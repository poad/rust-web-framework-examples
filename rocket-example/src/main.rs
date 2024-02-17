#[macro_use]
extern crate rocket;

use rocket::{
    fairing::AdHoc,
    http::Header,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HealthCheckResponse<'a> {
    status: &'a str,
}

#[get("/?<msg>")]
fn index(msg: &str) -> Json<Response> {
    let message = format!("Hello! {}", msg);
    Json(Response { message })
}

#[get("/")]
fn health_check<'a>() -> Json<HealthCheckResponse<'a>> {
    Json(HealthCheckResponse { status: "success" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_response("Add Headers", |_, res| {
            Box::pin(async move {
                res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
                res.set_header(Header::new("Access-Control-Allow-Methods", "*"));
                res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            })
        }))
        .mount("/", routes![index])
        .mount("/health", routes![health_check])
    }
