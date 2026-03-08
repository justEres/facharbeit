use facharbeit::lsp::backend::serve;

#[tokio::main]
async fn main() {
    serve().await;
}
