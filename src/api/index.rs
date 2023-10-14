use actix_web::{web, get, HttpResponse, Responder};
use serde::Serialize;
use super::books;
use super::genres;

#[derive(Serialize)]
struct HealthCheckResponse {
    message: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = HealthCheckResponse {
        message: String::from("The microservice is working fine."),
    };
    HttpResponse::Ok().json(response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("")
        .configure(books::config)
        .configure(genres::config)
        .service(health_check)
  );
}