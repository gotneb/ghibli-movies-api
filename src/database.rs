use std::env;
use std::io;

use mongodb::{Client, Cursor, bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}};
use bson::{Document, Bson};

use crate::movie::Movie;

pub struct Database {
    client: Client,
    db_name: String,
    coll_name: String,
}

impl Database {
    /// Creates a new instance of [Database]
    pub async fn new() -> Self {
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI not specified!");
        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not specified!");
        let coll_name = env::var("COLLECTION_NAME").expect("COLLECTION_NAME not specified!");

        let mut client_options = ClientOptions::parse(uri).await.unwrap();
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options).unwrap();

        Self {
            client,
            db_name,
            coll_name,
        }
    }

    /// Searches for many movies related to a given title into database
    pub async fn search(&self, title: &String) -> Option<Vec<Movie>> {
        // This query only works if an index is already created in MongoDB
        // Otherwise `find` method panics!
        let query = doc! { "$text": { "$search": title, } };
    
        let mut cursor: Cursor<Document> = self.client
            .database(&self.db_name)
            .collection(&self.coll_name)
            .find(Some(query), None)
            .await
            .unwrap();
    
        let mut movies = Vec::new();

        loop {
            if !cursor.advance().await.unwrap() {
                break
            }

            match cursor.deserialize_current() {
                Ok(doc) => {
                    if let Ok(movie) = bson::from_bson::<Movie>(Bson::Document(doc)) {
                        movies.push(movie);
                    }
                }
                Err(_) => (),
            }
        }

        if movies.len() > 0 {Some(movies)} else {None}
    }

    /// Find a movie by its id
    pub async fn find_id(&self, id: &String) -> Option<Movie> {
        let query = doc! { 
            "id": id
        };
    
        let result: Option<Document> = self.client
            .database(&self.db_name)
            .collection(&self.coll_name)
            .find_one(Some(query), None)
            .await
            .unwrap();
    
        match result {
            Some(doc) => {
                if let Ok(movie) = bson::from_bson::<Movie>(Bson::Document(doc)) {
                    Some(movie)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    /// Add a new manga movie into database
    pub async fn add(&self, movie: &Movie) -> Result<(), io::Error> {
        let movie = bson::to_document(&movie);

        match movie {
            Ok(movie) => {
                let _ = self.client
                .database(&self.db_name)
                .collection::<Document>(&self.coll_name)
                .insert_one(movie, None)
                .await
                .expect("Error adding movie!");

                Ok(())
            },
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, format!("MongoDB insertion error: {}", err))),
        }
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            db_name: self.db_name.clone(),
            coll_name: self.coll_name.clone(),
        }
    }
}