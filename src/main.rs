use axum::extract::Query;
use serde::Deserialize;
use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_manta = Router::new().route("/api", get(handler_manta));
    // region: Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_manta.into_make_service())
        .await
        .unwrap();
    // endregion: Start Server
}

#[derive(Debug, Deserialize)]
struct MantaParams {
    endpoint: Option<String>,
}

// region: Handler
async fn handler_manta(Query(params): Query<MantaParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_manta - {params:?}", "HANDLER");

    let endpoint = params.endpoint.as_deref().unwrap_or("c2b");
    Html(format!("<h1>API: {endpoint}!</h1>"))
}
// endregion: Handler
