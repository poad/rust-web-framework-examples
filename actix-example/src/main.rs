use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Request {
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[get("/")]
async fn index(req: web::Query<Request>) -> HttpResponse {
    HttpResponse::Ok().json(Response { message: format!("Hello! {}", req.msg) })
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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
