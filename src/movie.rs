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
    title_image: String,
    poster: String,
    alternative_poster: String,
    description: String,
    promotional_image: String,
    director: String,
    release_year: u32,
    duration: u32,
    mpaa: String,
    score: f64,
    trailer: String,
    genres: Vec<String>,
    gallery: Vec<String>,
}

impl Movie {
    /// Creates a new [Movie]
    pub fn from(&self) -> Self {
        Self {
            id: digest(self.title.clone()),
            title: self.title.clone(),
            original_title: self.original_title.clone(),
            alternative_title: self.alternative_title.clone(),
            title_image: self.title_image.clone(),
            poster: self.poster.clone(),
            alternative_poster: self.alternative_poster.clone(),
            description: self.description.clone(),
            promotional_image: self.promotional_image.clone(),
            director: self.director.clone(),
            release_year: self.release_year,
            duration: self.duration,
            mpaa: self.mpaa.clone(),
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
