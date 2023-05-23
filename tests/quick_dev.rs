#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    
    hc.do_get("/api").await?.print().await?;
    
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_send = hc.do_post(
        "/api/send",
        json!({
            "amount": 100,
            "receiver": "254746638248",
            "description": "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
            
        }),
    );
    
    req_create_send.await?.print().await?;
    hc.do_get("/api/send").await?.print().await?;
    // hc.do_delete("/api/send/delete/0").await?.print().await?;
    
    Ok(())
}
