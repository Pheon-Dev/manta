use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:  --- API Types

// C2B Simulate API Endpoint
#[derive(Debug, Clone, Serialize)]
pub struct C2BSimulateRequest {
    pub id: u64, // command ID
    pub amount: u32,
    pub msisdn: u64,         // phone number
    pub description: String, // bill ref number
    pub shortcode: u32,      // till or paybill number
}

#[derive(Deserialize)]
pub struct C2BSimulateRequestCreate {
    pub amount: u32,
    pub msisdn: u64,
    pub description: String,
    pub shortcode: u32,
}
// endregion:  --- API Types

// region:  --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    c2b_simulate_store: Arc<Mutex<Vec<Option<C2BSimulateRequest>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            c2b_simulate_store: Arc::default(),
        })
    }
}

// CRUD Implementations
impl ModelController {
    pub async fn c2b_simulate_create(
        &self,
        c2b_src: C2BSimulateRequestCreate,
    ) -> Result<C2BSimulateRequest> {
        let mut store = self.c2b_simulate_store.lock().unwrap();
        let id = store.len() as u64;
        let c2b_sim_req = C2BSimulateRequest {
            id,
            amount: c2b_src.amount,
            msisdn: c2b_src.msisdn,
            description: c2b_src.description,
            shortcode: c2b_src.shortcode,
        };
        store.push(Some(c2b_sim_req.clone()));

        Ok(c2b_sim_req)
    }

    pub async fn c2b_simulate_list(&self) -> Result<Vec<C2BSimulateRequest>> {
        let store = self.c2b_simulate_store.lock().unwrap();
        let c2b_sim_reqs = store.iter().filter_map(|t| t.clone()).collect();
        Ok(c2b_sim_reqs)
    }

    pub async fn c2b_simulate_delete(&self, id: u64) -> Result<C2BSimulateRequest> {
        let mut store = self.c2b_simulate_store.lock().unwrap();
        let c2b_sim_req = store.get_mut(id as usize).and_then(|t| t.take());

        c2b_sim_req.ok_or(Error::C2BSimulateDeleteFailIdNotFound { id })
    }

    // pub async fn c2b_simulate_get(&self, id: u64) -> Result<Option<C2BSimulateRequest>> {}
}

// endregion:  --- Model Controller
