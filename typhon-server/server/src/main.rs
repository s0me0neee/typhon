use anyhow::{Ok, Result};
use axum::{Json, Router, extract::State, routing::get};
use std::{
    env::{self},
    sync::Arc,
};
use terrain::World;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

// ts-rs export path from the module path, e.g. server::sim::war -> sim/war.ts
macro_rules! ts_file {
    () => {
        format!(
            "{}.ts",
            module_path!()
                .split_once("::")
                .map_or("main", |(_, m)| m)
                .replace("::", "/")
        )
    };
}

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
    let world = Arc::new(World::new(20, 10)?);
    let app = Router::new()
        .route("/", get(|| async { "Hello axum" }))
        .route("/health", get(|| async { "ok" }))
        .route("/view/world", get(view_world))
        .with_state(world)
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

async fn view_world(State(world): State<Arc<World>>) -> Json<Arc<World>> {
    Json(world)
}
