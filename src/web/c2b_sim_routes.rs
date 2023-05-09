use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::model::{C2BSimulateRequest, C2BSimulateRequestCreate, ModelController};
use crate::Result;

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
async fn create_c2b_sim_req(
    State(mc): State<ModelController>,
    Json(c2b_src): Json<C2BSimulateRequestCreate>,
) -> Result<Json<C2BSimulateRequest>> {
    println!("->> {:<12} - create_c2b_sim_req", "HANDLER");
    let c2b_sim_req = mc.c2b_simulate_create(c2b_src).await?;
    Ok(Json(c2b_sim_req))
}

async fn list_c2b_sim_req(
    State(mc): State<ModelController>,
) -> Result<Json<Vec<C2BSimulateRequest>>> {
    println!("->> {:<12} - list_c2b_sim_reqs", "HANDLER");
    let c2b_sim_reqs = mc.c2b_simulate_list().await?;
    Ok(Json(c2b_sim_reqs))
}

async fn delete_c2b_sim_req(
    State(mc): State<ModelController>,
    // ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<C2BSimulateRequest>> {
    println!("->> {:<12} - delete_c2b_sim_req", "HANDLER");
    // let c2b_sim_req = mc.c2b_simulate_delete(ctx, id).await?;
    let c2b_sim_req = mc.c2b_simulate_delete(id).await?;
    Ok(Json(c2b_sim_req))
}

// endregion: --- REST Handler
