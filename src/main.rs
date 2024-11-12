use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await?;
    let app = Router::new().route("/hello", get(|| async { "Hello, world" })).with_state(client);
    let listener = TcpListener::bind("127.0.0.1:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}