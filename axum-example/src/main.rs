use axum::{
    extract::{Json, Query},
    routing::get,
    Router,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

use std::collections::HashMap;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

async fn index(Query(params): Query<HashMap<String, String>>) -> Json<Response> {
    let message = format!("Hello! {}", params.get("msg").unwrap());
    Json(Response { message: message })
}

async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "success".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
