use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Deserialize, Serialize)]
pub struct Movie {
    #[serde(default)]
    id: String,
    // English title
    title: String,
    // Japanese title
    original_title: String,
    // Romaji title
    alternative_title: String,
    poster_title: String,
    poster: String,
    description: String,
    background_poster: String,
    director: String,
    release_year: u32,
    duration: u32,
    score: f64,
    trailer: String,
    genres: Vec<String>,
    gallery: Vec<String>,
}

impl Movie {
    /// Creates a new [Movie] with constraints
    pub fn from(&self) -> Self {
        Self {
            id: digest(self.title.clone()),
            title: self.title.clone(),
            original_title: self.title.clone(),
            alternative_title: self.alternative_title.clone(),
            poster_title: self.poster_title.clone(),
            poster: self.poster.clone(),
            description: self.description.clone(),
            background_poster: self.background_poster.clone(),
            director: self.director.clone(),
            release_year: self.release_year,
            duration: self.duration,
            score: self.score,
            trailer: self.trailer.clone(),
            genres: self.genres.clone(),
            gallery: self.gallery.clone(),
        }
    }
}

impl Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {} ({})", self.title, self.original_title, self.alternative_title)
    }
}
