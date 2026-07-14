use std::error::Error;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Deserialize;

use crate::companies::NewCompany;
use crate::schema::companies;

#[derive(Debug, Deserialize)]
struct CompanyCsvRow {
    company_name: String,
}

pub fn import_companies_from_csv(
    connection: &mut SqliteConnection,
    file_path: &str,
) -> Result<usize, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut inserted_count = 0;

    for result in reader.deserialize() {
        let row: CompanyCsvRow = result?;
        let company_name = row.company_name.trim();

        // Ignore an accidentally blank CSV row.
        if company_name.is_empty() {
            continue;
        }

        let new_company = NewCompany { company_name };

        inserted_count += diesel::insert_into(companies::table)
            .values(&new_company)
            .execute(connection)?;
    }

    Ok(inserted_count)
}