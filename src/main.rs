use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use diesel::SqliteConnection;
use std::sync::Mutex;

#[macro_use]
extern crate diesel;

mod api;
mod database;

struct AppState {
    connection: SqliteConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = database::db::establish_connection();
    let data = Data::new(Mutex::new(AppState { connection }));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(web::scope("/api").configure(api::index::config))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
