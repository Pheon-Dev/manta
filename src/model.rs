use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::{IntoParams, ToSchema};

// region:  --- API Types


// Send API Endpoint
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SendRequest {
    pub id: u64,
    pub cid: u64,
    pub amount: u32,
    pub receiver: String,
    pub description: String,
}

#[derive(Deserialize, ToSchema)]
pub struct SendRequestCreate {
    pub amount: u32,
    pub receiver: String,
    pub description: String,
}

// endregion:  --- API Types

// region:  --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    send_store: Arc<Mutex<Vec<Option<SendRequest>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            send_store: Arc::default(),
        })
    }
}

// CRUD Implementations
impl ModelController {

    pub async fn send_create(&self, ctx: Ctx, send_src: SendRequestCreate) -> Result<SendRequest> {
        let mut store = self.send_store.lock().unwrap();
        let id = store.len() as u64;
        let send_req = SendRequest {
            id,
            cid: ctx.user_id(),
            amount: send_src.amount,
            receiver: send_src.receiver,
            description: send_src.description,
        };
        store.push(Some(send_req.clone()));
        Ok(send_req)
    }

    pub async fn send_list(&self, _ctx: Ctx) -> Result<Vec<SendRequest>> {
        let store = self.send_store.lock().unwrap();
        let send_reqs = store.iter().filter_map(|t| t.clone()).collect();
        Ok(send_reqs)
    }


    pub async fn send_delete(&self, _ctx: Ctx, id: u64) -> Result<SendRequest> {

        let mut store = self.send_store.lock().unwrap();
        let send_req = store.get_mut(id as usize).and_then(|t| t.take());

send_req.ok_or(Error::SendDeleteFailIdNotFound { id })
    }
}

// endregion:  --- Model Controller
