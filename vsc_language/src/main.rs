use tower_lsp::{LspService, Server, Client};
use tower_lsp::lsp_types::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::LanguageServer;
// 仮定: parse_score は Result<_, ParseError> を返し、ParseError に line: Option<usize> がある
// ParseError 型を vec_score_drawer からインポートする必要があります。
use vec_score_drawer::parser::{parse_score, ParseError}; // ParseError をインポート (仮)

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _params: InitializedParams) { }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.validate(&params.text_document.uri, &params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // FULL sync なので、常に最初の変更が全文になる
        if let Some(change) = params.content_changes.first() {
             self.validate(&params.text_document.uri, &change.text).await;
        }
    }
}

impl Backend {
    /// text を parse_score に投げてエラーがあれば Diagnostic を返し、クライアントへ送信
    async fn validate(&self, uri: &Url, text: &str) {
        self.client.log_message(MessageType::INFO, format!("Validating URI: {}", uri)).await;

        let mut diagnostics = Vec::new();

        match parse_score(text) {
            Ok(_) => {
                self.client.log_message(MessageType::INFO, "Parse successful.").await;
                // Ok の場合は Diagnostic をクリアする (空の Vec を publish する)
            }
            Err(errs) => {
                for err in errs {
                    self.client.log_message(MessageType::ERROR, format!("Parse error: {}", err)).await;
                    let line_idx = err.line.unwrap_or(0);
                    let char_end = text.lines()
                                       .nth(line_idx)
                                       .map_or(0, |line_text| line_text.len()) as u32;
                    let range = Range {
                        start: Position { line: line_idx as u32, character: 0 },
                        end:   Position { line: line_idx as u32, character: char_end },
                    };
                    diagnostics.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: err.to_string(),
                        source: Some("VecScoreParser".to_string()),
                        ..Default::default()
                    });
                }
            }
        }

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    // Client を Backend に渡す
    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
}
