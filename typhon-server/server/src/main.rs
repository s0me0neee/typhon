use anyhow::{Ok, Result};
use axum::{Router, routing::get};
use std::env::{self};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
mod simulation;
mod terrain;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("server=debug,tower_http=info")),
        )
        .init();

    let port = env::var("PORT")?;
    let app = Router::new()
        .route("/", get(|| async { "Hello axum" }))
        .route("/health", get(|| async { "ok" }))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &port).await?;

    let elysia_port = env::args().collect::<Vec<_>>()[1].clone();

    info!(addr = %listener.local_addr()?, "sim server listening");
    info!(elysia_port, "configured llm server at");

    let serve = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            tracing::error!(?e, "axum died");
        }
    });
    simulation::simulation().await?;
    serve.await?;
    Ok(())
}
