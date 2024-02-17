use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    msg: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

#[get("/")]
async fn index(req: web::Query<Request>) -> HttpResponse {
    HttpResponse::Ok().json(Response { message: format!("Hello! {}", req.msg) })
}

#[get("/health")]
async fn health_chek() -> HttpResponse {
    HttpResponse::Ok().json(HealthCheckResponse { status: "success".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600),
        )
            .service(index)
            .service(health_chek)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
