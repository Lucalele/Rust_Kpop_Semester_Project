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