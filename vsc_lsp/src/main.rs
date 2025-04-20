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
                    TextDocumentSyncKind::FULL, // ← FULL に変更して試す
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
        // --- Debug Output Start ---
        self.client.log_message(MessageType::INFO, format!("Validating URI: {}", uri)).await;
        // Be cautious logging the full text if it can be very large
        // self.client.log_message(MessageType::INFO, format!("Text to validate:\n{}", text)).await;
        // --- Debug Output End ---

        let mut diagnostics = Vec::new();

        match parse_score(text) {
            Ok(_) => {
                // --- Debug Output ---
                self.client.log_message(MessageType::INFO, "Parse successful.").await;
                // Ok の場合は Diagnostic をクリアする
            }
            Err(err_msg) => {
                // --- Debug Output ---
                self.client.log_message(MessageType::ERROR, format!("Parse error: {}", err_msg)).await;

                // デフォルトは先頭行
                let mut line_idx = 0;
                let mut char_end = text.lines().next().unwrap_or("").len() as u32;

                // メッセージ中の "Line N" を探す
                if let Some(pos) = err_msg.find("Line ") {
                    let rest = &err_msg[pos + "Line ".len()..];
                    if let Some(num_str) = rest.split_whitespace().next() {
                        if let Ok(n) = num_str.parse::<usize>() {
                            if n > 0 {
                                line_idx = n - 1;
                                if let Some(line_text) = text.lines().nth(line_idx) {
                                    char_end = line_text.len() as u32;
                                }
                            }
                        }
                    }
                }

                let range = Range {
                    start: Position { line: line_idx as u32, character: 0 },
                    end:   Position { line: line_idx as u32, character: char_end },
                };
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err_msg,
                    source: Some("VecScoreParser".to_string()),
                    ..Default::default()
                });
            }
        }

        // Publish diagnostics (even if empty to clear previous errors)
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
