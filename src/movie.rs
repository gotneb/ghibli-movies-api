use std::fmt::Display;

use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Movie {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    poster: String,
    description: String,
    background_poster: String,
    director: String,
    release_year: u32,
    duration: u32,
    score: f64,
    genres: Vec<String>,
    gallery: Vec<String>,
}

impl Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.title, self.score, self.release_year)
    }
}