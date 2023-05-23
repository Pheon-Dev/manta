use crate::{
    error::{Error, Result},
    web,
};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login_api))
}

/// create .e.g '/api/login'
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginPayload,
    // params(LoginPayload),
    responses(
        (status = 200, description = "Login Success", body = LoginResponse),
         (status = 404, description = "Login Fail"),
    )
)]
async fn login_api(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - login_api", "HANDLER");

    // db logic authentication
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // success body
    let body = Json(json!({
        "action": "login",
        "success": true
    }));

    Ok(body)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub action: String,
    pub success: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
