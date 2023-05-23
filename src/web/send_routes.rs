use crate::ctx::Ctx;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{ModelController, SendRequest, SendRequestCreate};
use crate::Result;

/// Send API Endpoint
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/send", post(create_send_req).get(list_send_req))
        .route("/send/delete/:id", delete(delete_send_req))
        .with_state(mc)
}

/// CREATE a Send Request
/// e.g 'api/send'
#[utoipa::path(
    post,
    path = "/api/send",
    request_body = SendRequestCreate,
    responses((
        status = 200,
        body = [SendRequest],
        description = "Send Request Created",
    ), (status = 404, description = "Send Request Not Found"))
)]
async fn create_send_req(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(send_src): Json<SendRequestCreate>,
) -> Result<Json<SendRequest>> {
    println!("->> {:<12} - create_send_req", "HANDLER");
    let send_req = mc.send_create(ctx, send_src).await?;
    Ok(Json(send_req))
}

/// LIST Sent Requests
/// e.g 'api/send'
#[utoipa::path(
    get,
    path = "/api/send",
    responses((
        status = 200,
        body = [SendRequest],
        description = "Send Request List",
    ), (status = 404, description = "Send Request List Not Found"))
)]
async fn list_send_req(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<SendRequest>>> {
    println!("->> {:<12} - list_send_reqs", "HANDLER");
    let send_reqs = mc.send_list(ctx).await?;
    Ok(Json(send_reqs))
}

/// DELETE a Sent Request
/// e.g 'api/send/delete/:id'
#[utoipa::path(
    delete,
    path = "/api/send/delete/{id}",
    params(("id" = u64, Path, description = "Transaction ID")),
    responses((
        status = 200,
        description = "Request Deleted",
    ), (status = 404, description = "Request Failed to Delete"))
)]
async fn delete_send_req(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<SendRequest>> {
    println!("->> {:<12} - delete_send_req", "HANDLER");
    let send_req = mc.send_delete(ctx, id).await?;
    Ok(Json(send_req))
}
