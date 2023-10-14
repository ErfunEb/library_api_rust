use crate::{database::models::genres::Genre, AppState};
use actix_web::{
    delete, get,
    http::header::ContentType,
    patch, post,
    web::{self, Data},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize)]
struct GenreListResponse {
    status: bool,
    data: Vec<Genre>,
}

#[derive(Serialize)]
struct GenreResponse {
    status: bool,
    data: Genre,
}

#[derive(Deserialize, Debug)]
struct GenreParams {
    title: String,
    parent_id: Option<String>,
}

#[get("")]
async fn list(data: Data<Mutex<AppState>>) -> impl Responder {
    let state = data.lock().unwrap();
    let genres = Genre::list(&state.connection);
    HttpResponse::Ok().json(GenreListResponse {
        status: true,
        data: genres,
    })
}

#[post("")]
async fn create(body: web::Json<GenreParams>, data: Data<Mutex<AppState>>) -> impl Responder {
    let state = data.lock().unwrap();
    let genre = Genre::create(&state.connection, body.title.to_string());
    HttpResponse::Ok().json(GenreResponse {
        status: true,
        data: genre.unwrap(),
    })
}

#[patch("/{genre_id}")]
async fn update(
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
    body: web::Json<GenreParams>,
) -> impl Responder {
    let state = data.lock().unwrap();
    let genre_id: String = req.match_info().load().unwrap();
    let credentials = (body.title.clone(), body.parent_id.clone());
    let updated_genre = Genre::update(&state.connection, genre_id, credentials);
    HttpResponse::Ok().json(GenreResponse {
        status: true,
        data: updated_genre.unwrap(),
    })
}

#[delete("/{genre_id}")]
async fn delete(data: Data<Mutex<AppState>>, req: HttpRequest) -> impl Responder {
    let state = data.lock().unwrap();
    let genre_id: String = req.match_info().load().unwrap();
    Genre::delete(&state.connection, genre_id);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{\"status\":true}")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("genres")
            .service(list)
            .service(create)
            .service(update)
            .service(delete),
    );
}
