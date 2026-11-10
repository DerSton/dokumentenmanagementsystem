use axum::{
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 1. API-Endpunkte unter /api
    let api_router = Router::new()
        .route("/health", get(health_check));

    // 2. Gesamte App: /api -> API Router, alles andere -> Embedded Frontend
    let app = Router::new()
        .nest("/api", api_router)
        .merge(web::router());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("DMS Server gestartet auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "dms-backend"
    }))
}
