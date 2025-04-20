use tower_lsp::{LspService, Server, Client};
use tower_lsp::lsp_types::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::LanguageServer;
// 修正: 正しいクレート名をインポート
use vec_score_drawer::parser::parse_score;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    // `_params` に変更して警告を抑制
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
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
        let text = params.content_changes[0].text.clone();
        self.validate(&params.text_document.uri, &text).await;
    }
}

impl Backend {
    /// text を parse_score に投げてエラーがあれば Diagnostic を返し、クライアントへ送信
    async fn validate(&self, uri: &Url, text: &str) {
        let mut diagnostics = Vec::new();

        if let Err(err_msg) = parse_score(text) {
            // とりあえず document 全体の先頭行にエラーとみなして表示
            let range = Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: text.lines().next().unwrap_or("").len() as u32 },
            };
            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                message: err_msg,
                ..Default::default()
            });
        }

        // クライアントへ送信
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
