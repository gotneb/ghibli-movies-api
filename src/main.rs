mod movie;
mod database;

use database::Database;

fn main() {
    let db = Database::new();

    if let Some(movies) = db.search(&"rise".to_string()) {
        for m in movies {
            println!("{}", m);
        }
    }
}