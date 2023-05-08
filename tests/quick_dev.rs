#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/manta").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;
    // let req_login = hc.do_post(".api/login", json!({"username": "demo1", "pwd": "welcome"}));
    Ok(())
}
