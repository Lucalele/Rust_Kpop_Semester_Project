use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

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

        _ => panic!("Invalid database number: {database_number}. Use 0 through 7."),
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

pub fn initialize_database_zero(connection: &mut SqliteConnection) {
    connection
        .batch_execute(
            r#"
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS companies (
                company_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                company_name TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS labels (
                label_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                label_name TEXT NOT NULL,
                company_id INTEGER NOT NULL,

                FOREIGN KEY (company_id)
                    REFERENCES companies(company_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS idols (
                idol_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                idol_gender TEXT NOT NULL,
                is_soloist BOOLEAN
            );

            CREATE TABLE IF NOT EXISTS idol_names (
                idol_name_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                idol_id INTEGER NOT NULL,
                name TEXT NOT NULL,

                FOREIGN KEY (idol_id)
                    REFERENCES idols(idol_id)
                    ON DELETE CASCADE,

                UNIQUE (idol_id, name)
            );

            CREATE TABLE IF NOT EXISTS idol_groups (
                group_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                group_name TEXT NOT NULL,
                debut_date DATE,

                gender TEXT NOT NULL
                    CHECK (gender IN ('Male', 'Female', 'CoEd'))
            );

            CREATE TABLE IF NOT EXISTS subunits (
                subunit_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                subunit_name TEXT NOT NULL,
                parent_group_id INTEGER NOT NULL,
                debut_date DATE,

                gender TEXT NOT NULL
                    CHECK (gender IN ('Male', 'Female', 'CoEd')),

                FOREIGN KEY (parent_group_id)
                    REFERENCES idol_groups(group_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS project_groups (
                project_group_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                project_group_name TEXT NOT NULL,
                debut_date DATE,

                gender TEXT NOT NULL
                    CHECK (gender IN ('Male', 'Female', 'CoEd'))
            );

            CREATE TABLE IF NOT EXISTS albums (
                album_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                title TEXT NOT NULL,
                artist_id INTEGER NOT NULL,

                artist_type TEXT NOT NULL
                    CHECK (
                        artist_type IN (
                            'Group',
                            'Subunit',
                            'ProjectGroup',
                            'Soloist'
                        )
                    ),

                release_date DATE,
                language TEXT,
                version TEXT,

                UNIQUE (artist_id, artist_type, title, version)
            );

            CREATE TABLE IF NOT EXISTS idol_group_memberships (
                idol_id INTEGER NOT NULL,
                group_id INTEGER NOT NULL,

                PRIMARY KEY (idol_id, group_id),

                FOREIGN KEY (idol_id)
                    REFERENCES idols(idol_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (group_id)
                    REFERENCES idol_groups(group_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS idol_subunit_memberships (
                idol_id INTEGER NOT NULL,
                subunit_id INTEGER NOT NULL,

                PRIMARY KEY (idol_id, subunit_id),

                FOREIGN KEY (idol_id)
                    REFERENCES idols(idol_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (subunit_id)
                    REFERENCES subunits(subunit_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS idol_project_group_memberships (
                idol_id INTEGER NOT NULL,
                project_group_id INTEGER NOT NULL,

                PRIMARY KEY (idol_id, project_group_id),

                FOREIGN KEY (idol_id)
                    REFERENCES idols(idol_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (project_group_id)
                    REFERENCES project_groups(project_group_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS idol_companies (
                idol_id INTEGER NOT NULL,
                company_id INTEGER NOT NULL,

                PRIMARY KEY (idol_id, company_id),

                FOREIGN KEY (idol_id)
                    REFERENCES idols(idol_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (company_id)
                    REFERENCES companies(company_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS idol_labels (
                idol_id INTEGER NOT NULL,
                label_id INTEGER NOT NULL,

                PRIMARY KEY (idol_id, label_id),

                FOREIGN KEY (idol_id)
                    REFERENCES idols(idol_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (label_id)
                    REFERENCES labels(label_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS group_companies (
                group_id INTEGER NOT NULL,
                company_id INTEGER NOT NULL,

                PRIMARY KEY (group_id, company_id),

                FOREIGN KEY (group_id)
                    REFERENCES idol_groups(group_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (company_id)
                    REFERENCES companies(company_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS group_labels (
                group_id INTEGER NOT NULL,
                label_id INTEGER NOT NULL,

                PRIMARY KEY (group_id, label_id),

                FOREIGN KEY (group_id)
                    REFERENCES idol_groups(group_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (label_id)
                    REFERENCES labels(label_id)
                    ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS project_group_parents (
                project_group_id INTEGER NOT NULL,
                parent_group_id INTEGER NOT NULL,

                PRIMARY KEY (project_group_id, parent_group_id),

                FOREIGN KEY (project_group_id)
                    REFERENCES project_groups(project_group_id)
                    ON DELETE CASCADE,

                FOREIGN KEY (parent_group_id)
                    REFERENCES idol_groups(group_id)
                    ON DELETE CASCADE
            );
            "#,
        )
        .expect("Could not initialize database zero");
}

pub fn initialize_all_dbz() {
    let mut connection = establish_selected_connection(0);

    initialize_database_zero(&mut connection);

    println!("Initialized database{}.sqlite with all tables", 0);
}
