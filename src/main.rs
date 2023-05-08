use axum::extract::{Path, Query};
use serde::Deserialize;
use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let api_route = Router::new().merge(api_routes());

    // region: Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(api_route.into_make_service())
        .await
        .unwrap();
    // endregion: Start Server
}

// region: Routes
fn api_routes() -> Router {
    Router::new()
        .route("/api1", get(handler_endpoint))
        .route("/api2/:endpoint", get(handler_endpoints))
}

#[derive(Debug, Deserialize)]
struct MantaParams {
    endpoint: Option<String>,
}

// region: Handler

// .e.g '/api/c2b'
async fn handler_endpoints(Path(endpoint): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_endpoints - {endpoint:?}", "HANDLER");
    Html(format!("<h1>APIv2: {endpoint}</h1>"))
}

// .e.g '/api?endpoint=c2b'
async fn handler_endpoint(Query(params): Query<MantaParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_endpoint - {params:?}", "HANDLER");

    let endpoint = params.endpoint.as_deref().unwrap_or("c2b");
    Html(format!("<h1>APIv1: {endpoint}</h1>"))
}
// endregion: Handler
