use std::fs::File;
use std::io::{BufRead, BufReader};

use diesel::sqlite::SqliteConnection;
use crate::companies::insert_company;

pub fn import_database_zero(
    _connection: &mut SqliteConnection,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split('|').collect();

      match parts[0] {
    "COMPANY" => {
        if parts.len() != 2 {
            println!("Invalid COMPANY line: {}", line);
            continue;
        }

        insert_company(_connection, parts[1])?;

        println!("Added company: {}", parts[1]);
    }

    _ => {
        println!("Unknown command: {}", parts[0]);
    }
}
    }

    Ok(())
}