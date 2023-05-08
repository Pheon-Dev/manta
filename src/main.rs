use axum::extract::{Path, Query};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let api_route = Router::new()
        .merge(api_routes())
        .merge(web::login_routes::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(static_routes());

    // region: Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(api_route.into_make_service())
        .await
        .unwrap();
    // endregion: Start Server
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

// region: Routes
fn api_routes() -> Router {
    Router::new()
        .route("/api", get(handler_ep_query))
        .route("/api/:endpoint", get(handler_endpoint))
}

fn static_routes() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct MantaParams {
    endpoint: Option<String>,
}

// region: Handler

// .e.g '/api/c2b'
async fn handler_endpoint(Path(endpoint): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_endpoint - {endpoint:?}", "HANDLER");
    Html(format!("<h1>APIv2: {endpoint}</h1>"))
}

// .e.g '/api?endpoint=c2b'
async fn handler_ep_query(Query(params): Query<MantaParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_ep_query - {params:?}", "HANDLER");

    let endpoint = params.endpoint.as_deref().unwrap_or("c2b");
    Html(format!("<h1>APIv1: {endpoint}</h1>"))
}
// endregion: Handler
