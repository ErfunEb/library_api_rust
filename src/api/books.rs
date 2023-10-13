use actix_web::{web, get, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Books page")
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
      web::scope("books")
        .service(index)
  );
}