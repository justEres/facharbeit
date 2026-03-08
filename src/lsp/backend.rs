use std::collections::HashMap;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, HoverProviderCapability,
    InitializeParams, InitializeResult, LanguageString, MarkedString, MessageType, Position,
    Range,
    ServerCapabilities, TextDocumentContentChangeEvent, TextDocumentSyncCapability,
    TextDocumentSyncKind, Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::analysis::{SymbolKind, analyze_diagnostic, symbol_at};
use crate::lsp::source_map::SourceMap;
use crate::token::Span;

#[derive(Debug, Clone)]
struct DocumentState {
    source: String,
    source_map: SourceMap,
}

impl DocumentState {
    fn new(source: String) -> Self {
        let source_map = SourceMap::new(&source);
        Self { source, source_map }
    }

    fn update(&mut self, changes: &[TextDocumentContentChangeEvent]) {
        if let Some(change) = changes.last() {
            self.source = change.text.clone();
            self.source_map = SourceMap::new(&self.source);
        }
    }
}

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: RwLock<HashMap<Url, DocumentState>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: RwLock::new(HashMap::new()),
        }
    }

    async fn publish_diagnostics(&self, uri: Url, source: &str) {
        let source_map = SourceMap::new(source);
        let diagnostics = analyze_diagnostic(source)
            .map(|diagnostic| {
                let severity = if diagnostic.span.is_some() {
                    DiagnosticSeverity::ERROR
                } else {
                    DiagnosticSeverity::WARNING
                };
                let range_span = diagnostic
                    .span
                    .clone()
                    .unwrap_or_else(|| source_map.fallback_span());
                vec![Diagnostic {
                    range: span_to_range(range_span, &source_map),
                    severity: Some(severity),
                    source: Some("eres".to_string()),
                    message: diagnostic.message,
                    ..Diagnostic::default()
                }]
            })
            .unwrap_or_default();
        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: tower_lsp::lsp_types::InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "eres language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        let state = DocumentState::new(document.text.clone());
        self.documents
            .write()
            .await
            .insert(document.uri.clone(), state);
        self.publish_diagnostics(document.uri, &document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let source = {
            let mut docs = self.documents.write().await;
            let entry = docs
                .entry(uri.clone())
                .or_insert_with(|| DocumentState::new(String::new()));
            entry.update(&params.content_changes);
            entry.source.clone()
        };
        self.publish_diagnostics(uri, &source).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().await.remove(&params.text_document.uri);
        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let text_document_position = params.text_document_position_params;
        let uri = text_document_position.text_document.uri;
        let position = text_document_position.position;

        let docs = self.documents.read().await;
        let Some(document) = docs.get(&uri) else {
            return Ok(None);
        };
        let offset = document
            .source_map
            .position_to_offset(position.line, position.character);

        let symbol = match symbol_at(&document.source, offset) {
            Ok(symbol) => symbol,
            Err(_) => return Ok(None),
        };
        let Some(symbol) = symbol else {
            return Ok(None);
        };

        let label = match symbol.kind {
            SymbolKind::Function => "Function",
            SymbolKind::Parameter => "Parameter",
            SymbolKind::Local => "Local",
            SymbolKind::Struct => "Struct",
            SymbolKind::Enum => "Enum",
        };

        Ok(Some(Hover {
            contents: HoverContents::Array(vec![
                MarkedString::String(label.to_string()),
                MarkedString::LanguageString(LanguageString {
                    language: "eres".to_string(),
                    value: symbol.detail.clone(),
                }),
            ]),
            range: Some(span_to_range(symbol.span, &document.source_map)),
        }))
    }
}

pub async fn serve() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

fn span_to_range(span: Span, source_map: &SourceMap) -> Range {
    let (start_line, start_col) = source_map.offset_to_position(span.start);
    let (end_line, end_col) = source_map.offset_to_position(span.end.max(span.start + 1));
    Range {
        start: Position {
            line: start_line,
            character: start_col,
        },
        end: Position {
            line: end_line,
            character: end_col,
        },
    }
}
