use actix_web::{web, App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").configure(api::index::config),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
