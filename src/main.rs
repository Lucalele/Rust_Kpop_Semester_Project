use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fs;
use std::fs::File;

use Rust_Kpop_Semester_Project::importer::import_database_zero;

fn main() {
    println!("Starting K-Pop Archive");

    init_tapedecks();

    // --- MAIN DATABASE CHECK ---
    let main_db = "src/database.sqlite";
    if !fs::metadata(main_db).is_ok() {
        eprintln!("Main database not found at {main_db}");
        eprintln!("Run: diesel migration run");
        return;
    }

    let mut conn = match SqliteConnection::establish(main_db) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to main DB: {e}");
            eprintln!("Run: diesel migration run");
            return;
        }
    };

    println!("Main database OK");

    println!("Running importer with src/album.txt");

    if let Err(e) = import_database_zero(&mut conn, "src/album.txt") {
        eprintln!("Importer failed: {e}");
        return;
    }

    println!("Import complete");
}

fn init_tapedecks() {
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;

    let schema = "
        CREATE TABLE IF NOT EXISTS albums_alt (
            album_id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            artist TEXT NOT NULL
        );
    ";

    for i in 1..=7 {
        let path = format!("src/database{i}.sqlite");

        // Create file if missing
        if !fs::metadata(&path).is_ok() {
            File::create(&path).expect("Failed to create Tape Deck database file");
        }

        // Apply schema
        let mut conn =
            SqliteConnection::establish(&path).expect("Failed to connect to Tape Deck database");

        diesel::sql_query(schema)
            .execute(&mut conn)
            .expect("Failed to apply Tape Deck schema");
    }
}
