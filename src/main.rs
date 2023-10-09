mod movie;
mod database;

use std::env;

use actix_web::{
    get, post, web, App, Error, HttpResponse, HttpServer, error::ErrorNotFound, 
};
use database::Database;
use serde::{Deserialize, Serialize};

use crate::movie::Movie;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

struct AppState {
    database: Database,
}

#[get("/movie/{searchTitle}")]
async fn get_movie(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let search_title = path.into_inner();

    if search_title.len() == 0 { return Err(ErrorNotFound("No movie found")) }

    match &data.database
          .search(&search_title)
          .await {
        Some(movie) => Ok(HttpResponse::Ok().json(movie)),
        None => {
            Err(ErrorNotFound("No movie found"))
        }
    }
}

#[post("/movie/add")]
async fn add_movie(info: web::Json<Movie>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let movie = Movie::from(&info.0);
    if let Ok(()) = data.database.add(&movie).await {
        Ok(HttpResponse::Ok().body("Movie added"))
    } else {
        Err(actix_web::error::ErrorInternalServerError(""))
    }
}

#[get("/movie/id/{id}")]
async fn get_movie_by_id(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    if id.len() == 0 { return Err(ErrorNotFound("No movie found")) }

    match &data.database.find_id(&id).await {
        Some(movie) => Ok(HttpResponse::Ok().json(movie)),
        None => {
            Err(ErrorNotFound("No movie found"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new().await;
    let port = env::var("PORT").expect("PORT not specified!");
    let port = port.parse::<u16>().unwrap();

    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                database: db.clone(),
            }))
            .service(get_movie)
            .service(add_movie)
            .service(get_movie_by_id)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await;

    result
}
