use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_manta = Router::new().route("/manta", get(handler_manta));
    // region: Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_manta.into_make_service())
        .await
        .unwrap();
    // endregion: Start Server

    // region: Handler
    async fn handler_manta() -> impl IntoResponse {
        println!("->> {:<12} - handler_manta", "HANDLER");
        Html("Hello <strong>Manta Ray!<strong>")
    }
    // endregion: Handler
}
