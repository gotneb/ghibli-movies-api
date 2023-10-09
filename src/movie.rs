use std::{fmt::Display, io::{Error, ErrorKind}};

use sha256::digest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Movie {
    #[serde(default)]
    id: String,
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

impl Movie {
    /// Creates a new [Movie] with constraints
    pub fn build(
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
    ) -> Result<Self, Error> {
        let texts = vec![&title, &poster, &description, &background_poster, &director];
        let are_empty_texts = texts.iter().any(|text| text.len() == 0);
        
        // Verifies if all fields are right
        if  are_empty_texts || 
            gallery.len() < 9 || 
            score <= 0. || 
            genres.len() <= 0 {
            return Err(Error::new(
                ErrorKind::Other, format!("Can't add movie. One or more fields are empties or negatives")
            ))
        }

        Ok(Self {
            id: digest(title.clone()),
            title,
            poster,
            background_poster,
            description,
            director,
            release_year,
            duration,
            score,
            genres,
            gallery,
        })
    }
}

impl Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.title, self.score, self.release_year)
    }
}
