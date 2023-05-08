use crate::error::{Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/auth/login", post(login_api))
}

async fn login_api(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - login_api", "HANDLER");

    // db logic authentication
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // set cookies

    // success body
    let body = Json(json!({"result": {"success": true}}));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
