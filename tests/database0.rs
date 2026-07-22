use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Text};

// Replace this with the exact package name from Cargo.toml.
// Rust converts hyphens in package names to underscores.
use Rust_Kpop_Semester_Project::database;

#[derive(Debug, QueryableByName)]
struct IntegrityResult {
    #[diesel(sql_type = Text)]
    integrity_check: String,
}

#[derive(Debug, QueryableByName)]
struct ForeignKeysStatus {
    #[diesel(sql_type = Integer)]
    foreign_keys: i32,
}

#[derive(Debug, QueryableByName)]
struct TableName {
    #[diesel(sql_type = Text)]
    name: String,
}

#[derive(Debug, QueryableByName)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[test]
fn database_zero_is_valid() {
    database::initialize_all_dbz();

    let mut connection = database::establish_selected_connection(0);

    let foreign_keys: ForeignKeysStatus =
        diesel::sql_query("PRAGMA foreign_keys;")
            .get_result(&mut connection)
            .expect("Could not check foreign-key status");

    assert_eq!(
        foreign_keys.foreign_keys, 1,
        "Foreign-key enforcement is disabled"
    );

    let integrity: IntegrityResult =
        diesel::sql_query("PRAGMA integrity_check;")
            .get_result(&mut connection)
            .expect("Could not run integrity check");

    assert_eq!(
        integrity.integrity_check, "ok",
        "SQLite integrity check failed"
    );

    let violations: CountResult = diesel::sql_query(
        "SELECT COUNT(*) AS count
         FROM pragma_foreign_key_check;",
    )
    .get_result(&mut connection)
    .expect("Could not run foreign-key check");

    assert_eq!(
        violations.count, 0,
        "Database contains foreign-key violations"
    );

    let tables: Vec<TableName> = diesel::sql_query(
        "SELECT name
         FROM sqlite_master
         WHERE type = 'table'
           AND name NOT LIKE 'sqlite_%'
           AND name != '__diesel_schema_migrations'
         ORDER BY name;",
    )
    .load(&mut connection)
    .expect("Could not retrieve the table list");

    assert!(
        !tables.is_empty(),
        "database0.sqlite contains no application tables"
    );

    for table in tables {
        let escaped_name = table.name.replace('"', "\"\"");

        let query = format!(
            "SELECT COUNT(*) AS count FROM \"{}\";",
            escaped_name
        );

        let result: CountResult = diesel::sql_query(query)
            .get_result(&mut connection)
            .unwrap_or_else(|error| {
                panic!(
                    "Table '{}' could not be queried: {}",
                    table.name, error
                )
            });

        println!("{}: {} row(s)", table.name, result.count);
    }
}