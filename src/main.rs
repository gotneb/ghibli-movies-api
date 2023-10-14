mod movie;
mod database;

use std::env;
use log::info;

use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, error::ErrorNotFound, Responder};
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

#[get("/")]
async fn greeting() -> impl Responder {
    info!("handling / (root)");
    HttpResponse::Ok().body("Hello! The server is working ;)")
}

#[get("/movies/search/{searchTitle}")]
async fn get_movie(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let search_title = path.into_inner();
    info!("handling /movie/search/{}", search_title);

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

#[get("/movies/random")]
async fn get_random_movie(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    info!("handling /movies/random");

    if let Some(movie) = data.database.get_random().await {
        Ok(HttpResponse::Ok().json(movie))
    } else {
        Err(ErrorNotFound("No movie found"))
    }
}

#[get("/movies/all")]
async fn get_all_movies(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    info!("handling /movies/all");

    if let Some(movies) = &data.database.all_movies().await {
        Ok(HttpResponse::Ok().json(movies))
    } else {
        Err(ErrorNotFound("No movie found"))
    }
}

#[post("/movies/add")]
async fn add_movie(info: web::Json<Movie>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    info!("handling /movies/add");

    let movie = Movie::from(&info.0);

    if let Ok(()) = data.database.add(&movie).await {
        Ok(HttpResponse::Ok().body("Movie added"))
    } else {
        Err(actix_web::error::ErrorInternalServerError(""))
    }
}

#[get("/movies/get/{id}")]
async fn get_movie_by_id(path: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    info!("handling /movie/get/{}", id);

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
    env_logger::init();

    let db = Database::new().await;
    let port = env::var("PORT").expect("PORT not specified!");
    let port = port.parse::<u16>().unwrap();

    info!("server successfully started at port: {}", port);

    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                database: db.clone(),
            }))
            .wrap(Cors::permissive()) // Apply the Cors middleware
            .service(greeting)
            .service(get_movie)
            .service(add_movie)
            .service(get_movie_by_id)
            .service(get_all_movies)
            .service(get_random_movie)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await;

    result
}