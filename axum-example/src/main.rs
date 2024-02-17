use axum::{
    routing::get,
    http::StatusCode,
    Router,
    extract::{
        Query,
        Json
    },
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

use std::collections::HashMap;

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn index(Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Response>) {
    let message = format!("Hello! {}", params.get("msg").unwrap());
    (StatusCode::CREATED, Json(Response { message: message }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
