use anyhow::Result;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Router, extract::State, routing::get};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod simulation;
mod terrain;

use simulation::{Simulation, Status};

type SharedSim = Arc<RwLock<Simulation>>;

// ts-rs export path from the module path server::sim::war → sim/war.ts
#[macro_export]
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

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new("server=debug,tower_http=info")
            }),
        )
        .init();

    let port = env::var("PORT")?;
    let elysia_port = env::args().nth(1).unwrap_or_default();

    let conf = terrain::WorldGenConf {
        perlin_noise_seed: rand::random(),
        ..Default::default()
    };
    info!(seed = conf.perlin_noise_seed, "generating world");
    let world = terrain::World::new(80, 45, &conf)?;
    let sim: SharedSim = Arc::new(RwLock::new(Simulation::new(world)));

    let api = Router::new()
        .route("/", get(|| async { "" }))
        .route("/view/world", get(world_view))
        .route("/view/status", get(status_view))
        .route("/step", post(step))
        .with_state(sim.clone())
        .layer(TraceLayer::new_for_http());

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(api);

    let listener =
        tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await?;

    info!(addr = %listener.local_addr()?, "sim server listening");
    info!(elysia_port, "configured llm server at");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn world_view(State(sim): State<SharedSim>) -> axum::response::Response {
    axum::Json(&sim.read().await.world).into_response()
}

async fn status_view(State(sim): State<SharedSim>) -> axum::Json<Status> {
    axum::Json(sim.read().await.status())
}

async fn step(State(sim): State<SharedSim>) -> axum::Json<Status> {
    let mut s = sim.write().await;
    s.step();
    axum::Json(s.status())
}
