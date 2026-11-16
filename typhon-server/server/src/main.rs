use anyhow::{Ok, Result};
use axum::{Router, routing::get};
use std::env::{self};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("server=debug,tower_http=debug")),
        )
        .init();

    let port = env::var("PORT")?;
    let app = Router::new()
        .route("/", get(|| async { "Hello axum" }))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &port).await?;

    let elysia_port = env::args().collect::<Vec<_>>()[1].clone();

    info!(addr = %listener.local_addr()?, "sim server listening");
    info!(elysia_port, "configured llm server at");
    axum::serve(listener, app).await?;

    let text = reqwest::get("0.0.0.0:".to_string() + &elysia_port).await?;
    info!(?text, "from llm server got");

    Ok(())
}
