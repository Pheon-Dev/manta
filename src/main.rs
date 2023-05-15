#![allow(unused)]
use crate::model::ModelController;
use axum::extract::{Path, Query};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

pub use self::error::{Error, Result};

use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise Model Controller
    let mc = ModelController::new().await?;

    let routes_apis = web::c2b_sim_routes::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let api_route = Router::new()
        .merge(api_routes())
        .merge(web::login_routes::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(static_routes());

    // region: Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(api_route.into_make_service())
        .await
        .unwrap();
    // endregion: Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- build a new response if there is a client error
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                    "error": {
                    "type": client_error.as_ref(),
                    "req_uuid":uuid.to_string(),
                }
            });

            println!("  ->> client_error_body: {client_error_body}");

            // build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // -- TODO: build and log the server log line
    println!("  ->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
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
    Html(format!("<h1>API: {endpoint}</h1>"))
}

// .e.g '/api?endpoint=c2b'
async fn handler_ep_query(Query(params): Query<MantaParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_ep_query - {params:?}", "HANDLER");

    let endpoint = params.endpoint.as_deref().unwrap_or("c2b");
    Html(format!("<h1>API: {endpoint}</h1>"))
}
// endregion: Handler
