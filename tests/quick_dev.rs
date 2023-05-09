#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/api").await?.print().await?;
    hc.do_get("/api?endpoint=c2b").await?.print().await?;
    hc.do_get("/api/b2c").await?.print().await?;
    // hc.do_get("/tests/quick_dev.rs").await?.print().await?;
    let req_login = hc.do_post(
        "/auth/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_c2b_simulate = hc.do_post(
        "/api/c2b/simulate",
        json!({
            "amount": 100,
            "msisdn": "254746638248",
            "description": "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
            "shortcode": 785868,
            
        }),
    );
    
    req_create_c2b_simulate.await?.print().await?;
    hc.do_get("/api/c2b/simulate").await?.print().await?;
    // hc.do_delete("/api/c2b/simulate/0").await?.print().await?;
    
    Ok(())
}
