use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    SqliteConnection::establish(&database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_1_connection() -> SqliteConnection {
    let database_url = "database1.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_2_connection() -> SqliteConnection {
    let database_url = "database2.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_3_connection() -> SqliteConnection {
    let database_url = "database3.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_4_connection() -> SqliteConnection {
    let database_url = "database4.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_5_connection() -> SqliteConnection {
    let database_url = "database5.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_6_connection() -> SqliteConnection {
    let database_url = "database6.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_database_7_connection() -> SqliteConnection {
    let database_url = "database7.sqlite";

    SqliteConnection::establish(database_url).unwrap_or_else(|error| {
        panic!("Could not connect to {database_url}: {error}");
    })
}

pub fn establish_selected_connection(database_number: u8) -> SqliteConnection {
    match database_number {
        // Special main archive, configured through .env
        0 => establish_connection(),

        // User-created rotation databases
        1 => establish_database_1_connection(),
        2 => establish_database_2_connection(),
        3 => establish_database_3_connection(),
        4 => establish_database_4_connection(),
        5 => establish_database_5_connection(),
        6 => establish_database_6_connection(),
        7 => establish_database_7_connection(),

        _ => panic!(
            "Invalid database number: {database_number}. Use 0 through 7."
        ),
    }
}

pub fn initialize_tape_deck(connection: &mut SqliteConnection) {
    connection
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS albums (
                album_id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                artist_id INTEGER NOT NULL,
                artist_type TEXT NOT NULL,
                release_date DATE,
                language TEXT,
                version TEXT
            );
            "#,
        )
        .expect("Could not create albums table");
}

pub fn initialize_all_tape_decks() {
    for database_number in 1..=7 {
        let mut connection = establish_selected_connection(database_number);

        initialize_tape_deck(&mut connection);

        println!(
            "Initialized database{}.sqlite with the albums table",
            database_number
        );
    }
}