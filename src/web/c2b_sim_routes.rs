use crate::ctx::Ctx;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{C2BSimulateRequest, C2BSimulateRequestCreate, ModelController};
use crate::Result;

/// Sample API Endpoint
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route(
            "/c2b/simulate",
            post(create_c2b_sim_req).get(list_c2b_sim_req),
        )
        .route("/c2b/simulate/:id", delete(delete_c2b_sim_req))
        .with_state(mc)
}

// region: --- REST Handler
/// create .e.g '/api/c2b/simulate'
#[utoipa::path(
    post,
    path = "/api/c2b/simulate",
    request_body = C2BSimulateRequestCreate,
    // params(C2BSimulateRequestCreate),
    responses((
        status = 200,
        body = [C2BSimulateRequest]
    ), (status = 404))
)]
async fn create_c2b_sim_req(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(c2b_src): Json<C2BSimulateRequestCreate>,
) -> Result<Json<C2BSimulateRequest>> {
    println!("->> {:<12} - create_c2b_sim_req", "HANDLER");
    let c2b_sim_req = mc.c2b_simulate_create(ctx, c2b_src).await?;
    Ok(Json(c2b_sim_req))
}

/// list .e.g '/api/c2b/simulate'
#[utoipa::path(
    get,
    path = "/api/c2b/simulate",
    responses((
        status = 200,
        body = [C2BSimulateRequest]
    ), (status = 404))
)]
async fn list_c2b_sim_req(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<C2BSimulateRequest>>> {
    println!("->> {:<12} - list_c2b_sim_reqs", "HANDLER");
    let c2b_sim_reqs = mc.c2b_simulate_list(ctx).await?;
    Ok(Json(c2b_sim_reqs))
}

/// delete .e.g '/api/c2b/simulate/:id'
#[utoipa::path(
    delete,
    path = "/api/c2b/simulate/{id}",
    params(("id"= u64, Path, description = "Transaction ID")),
    responses((
        status = 200,
        // body = [ModelController]
    ), (status = 404))
)]
async fn delete_c2b_sim_req(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<C2BSimulateRequest>> {
    println!("->> {:<12} - delete_c2b_sim_req", "HANDLER");
    let c2b_sim_req = mc.c2b_simulate_delete(ctx, id).await?;
    Ok(Json(c2b_sim_req))
}

// endregion: --- REST Handler
