# Manta

> M-PESA RESTful API but in Rust.

***

<!---->

## Technologies used

### Dependencies

*   async-trait = "0.1.68"
*   axum = "0.6.18"
*   lazy-regex = "2.5.0"
*   serde = { version = "1.0.162", features = \["derive"] }
*   serde\_json = "1.0.96"
*   strum\_macros = "0.24.3"
*   tokio = { version = "1.28.0", features = \["full"] }
*   tower-cookies = "0.9.0"
*   tower-http = { version = "0.4.0", features = \["fs"] }
*   uuid = { version = "1.3.2", features = \["v4", "fast-rng"] }

### Developer Dependencies

*   anyhow = "1.0.71"
*   httpc-test = "0.1.1"

***

<!---->

## Objectives

*   API Login

*   Response Mappers

*   Cookies

*   REST API (c2b, b2c, etc)

*   API Models

<!---->

*   Authentication Middlewares

*   Extractors (ctx)

*   Ctx from REST to API Models

*   Ctx Middleware Resolvers

*   Error Handling

*   Testing

*   Server Logging
