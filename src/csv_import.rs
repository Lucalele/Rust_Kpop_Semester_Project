use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::schema::{
    albums, companies, group_companies, group_labels, idol_companies,
    idol_group_memberships, idol_groups, idol_labels, idol_names,
    idol_project_group_memberships, idol_subunit_memberships, idols, labels,
    project_group_parents, project_groups, subunits,
};

#[derive(Debug, Default)]
pub struct ImportSummary {
    pub files_read: usize,
    pub rows_inserted: usize,
    pub skipped_files: Vec<&'static str>,
    pub table_counts: Vec<(&'static str, usize)>,
}

impl ImportSummary {
    fn record(&mut self, file_name: &'static str, count: Option<usize>) {
        match count {
            Some(count) => {
                self.files_read += 1;
                self.rows_inserted += count;
                self.table_counts.push((file_name, count));
            }
            None => self.skipped_files.push(file_name),
        }
    }
}

#[derive(Debug)]
struct CsvImportError(String);

impl fmt::Display for CsvImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for CsvImportError {}

fn parse_date(value: Option<String>, file_name: &str, row_number: usize) -> Result<Option<NaiveDate>, Box<dyn Error>> {
    let Some(value) = value else { return Ok(None) };
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }

    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map(Some)
        .map_err(|error| {
            Box::new(CsvImportError(format!(
                "{file_name}, row {row_number}: invalid date '{value}'. Use YYYY-MM-DD: {error}"
            ))) as Box<dyn Error>
        })
}

fn read_optional_csv<T: DeserializeOwned>(directory: &Path, file_name: &'static str) -> Result<Option<Vec<T>>, Box<dyn Error>> {
    let path = directory.join(file_name);
    if !path.exists() {
        return Ok(None);
    }

    let file = File::open(&path)?;
    let mut reader = csv::ReaderBuilder::new().trim(csv::Trim::All).from_reader(file);
    let mut rows = Vec::new();

    for (index, result) in reader.deserialize().enumerate() {
        let row = result.map_err(|error| {
            CsvImportError(format!("{} row {}: {error}", path.display(), index + 2))
        })?;
        rows.push(row);
    }

    Ok(Some(rows))
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = companies)]
struct CompanyRow {
    company_id: i32,
    #[serde(alias = "company")]
    company_name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = labels)]
struct LabelRow {
    label_id: i32,
    label_name: String,
    company_id: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idols)]
struct IdolRow {
    idol_id: i32,
    idol_gender: String,
    is_soloist: Option<bool>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idol_names)]
struct IdolNameRow {
    idol_name_id: i32,
    idol_id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct IdolGroupCsvRow {
    group_id: i32,
    group_name: String,
    debut_date: Option<String>,
    gender: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = idol_groups)]
struct IdolGroupInsertRow {
    group_id: i32,
    group_name: String,
    debut_date: Option<NaiveDate>,
    gender: String,
}

#[derive(Debug, Deserialize)]
struct SubunitCsvRow {
    subunit_id: i32,
    subunit_name: String,
    parent_group_id: i32,
    debut_date: Option<String>,
    gender: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = subunits)]
struct SubunitInsertRow {
    subunit_id: i32,
    subunit_name: String,
    parent_group_id: i32,
    debut_date: Option<NaiveDate>,
    gender: String,
}

#[derive(Debug, Deserialize)]
struct ProjectGroupCsvRow {
    project_group_id: i32,
    project_group_name: String,
    debut_date: Option<String>,
    gender: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = project_groups)]
struct ProjectGroupInsertRow {
    project_group_id: i32,
    project_group_name: String,
    debut_date: Option<NaiveDate>,
    gender: String,
}

#[derive(Debug, Deserialize)]
struct AlbumCsvRow {
    album_id: i32,
    title: String,
    artist_id: i32,
    release_date: Option<String>,
    language: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = albums)]
struct AlbumInsertRow {
    album_id: i32,
    title: String,
    artist_id: i32,
    release_date: Option<NaiveDate>,
    language: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = group_companies)]
struct GroupCompanyRow { group_id: i32, company_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = group_labels)]
struct GroupLabelRow { group_id: i32, label_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = project_group_parents)]
struct ProjectGroupParentRow { project_group_id: i32, parent_group_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idol_group_memberships)]
struct IdolGroupMembershipRow { idol_id: i32, group_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idol_subunit_memberships)]
struct IdolSubunitMembershipRow { idol_id: i32, subunit_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idol_project_group_memberships)]
struct IdolProjectGroupMembershipRow { idol_id: i32, project_group_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idol_companies)]
struct IdolCompanyRow { idol_id: i32, company_id: i32 }

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = idol_labels)]
struct IdolLabelRow { idol_id: i32, label_id: i32 }

pub fn import_all_csvs(connection: &mut SqliteConnection, directory: impl AsRef<Path>) -> Result<ImportSummary, Box<dyn Error>> {
    let directory: PathBuf = directory.as_ref().to_path_buf();
    if !directory.is_dir() {
        return Err(Box::new(CsvImportError(format!(
            "CSV directory does not exist: {}",
            directory.display()
        ))));
    }

    connection.batch_execute("PRAGMA foreign_keys = ON;")?;

    let summary = connection.transaction::<ImportSummary, Box<dyn Error>, _>(|connection| {
        let mut summary = ImportSummary::default();

        macro_rules! import_plain {
            ($file:literal, $row:ty, $table:path) => {{
                let rows = read_optional_csv::<$row>(&directory, $file)?;
                let count = match rows {
                    Some(rows) => {
                        let mut count = 0;
                        for row in rows {
                            count += diesel::insert_into($table)
                                .values(&row)
                                .execute(connection)?;
                        }
                        Some(count)
                    }
                    None => None,
                };
                summary.record($file, count);
            }};
        }

        // Parent tables first.
        import_plain!("companies.csv", CompanyRow, companies::table);
        import_plain!("labels.csv", LabelRow, labels::table);
        import_plain!("idols.csv", IdolRow, idols::table);
        import_plain!("idol_names.csv", IdolNameRow, idol_names::table);

        let group_rows = read_optional_csv::<IdolGroupCsvRow>(&directory, "idol_groups.csv")?;
        let group_count = match group_rows {
            Some(rows) => {
                let mut count = 0;
                for (index, row) in rows.into_iter().enumerate() {
                    let insert = IdolGroupInsertRow {
                        group_id: row.group_id,
                        group_name: row.group_name,
                        debut_date: parse_date(row.debut_date, "idol_groups.csv", index + 2)?,
                        gender: row.gender,
                    };
                    count += diesel::insert_into(idol_groups::table).values(&insert).execute(connection)?;
                }
                Some(count)
            }
            None => None,
        };
        summary.record("idol_groups.csv", group_count);

        let subunit_rows = read_optional_csv::<SubunitCsvRow>(&directory, "subunits.csv")?;
        let subunit_count = match subunit_rows {
            Some(rows) => {
                let mut count = 0;
                for (index, row) in rows.into_iter().enumerate() {
                    let insert = SubunitInsertRow {
                        subunit_id: row.subunit_id,
                        subunit_name: row.subunit_name,
                        parent_group_id: row.parent_group_id,
                        debut_date: parse_date(row.debut_date, "subunits.csv", index + 2)?,
                        gender: row.gender,
                    };
                    count += diesel::insert_into(subunits::table).values(&insert).execute(connection)?;
                }
                Some(count)
            }
            None => None,
        };
        summary.record("subunits.csv", subunit_count);

        let project_rows = read_optional_csv::<ProjectGroupCsvRow>(&directory, "project_groups.csv")?;
        let project_count = match project_rows {
            Some(rows) => {
                let mut count = 0;
                for (index, row) in rows.into_iter().enumerate() {
                    let insert = ProjectGroupInsertRow {
                        project_group_id: row.project_group_id,
                        project_group_name: row.project_group_name,
                        debut_date: parse_date(row.debut_date, "project_groups.csv", index + 2)?,
                        gender: row.gender,
                    };
                    count += diesel::insert_into(project_groups::table).values(&insert).execute(connection)?;
                }
                Some(count)
            }
            None => None,
        };
        summary.record("project_groups.csv", project_count);

        let album_rows = read_optional_csv::<AlbumCsvRow>(&directory, "albums.csv")?;
        let album_count = match album_rows {
            Some(rows) => {
                let mut count = 0;
                for (index, row) in rows.into_iter().enumerate() {
                    let insert = AlbumInsertRow {
                        album_id: row.album_id,
                        title: row.title,
                        artist_id: row.artist_id,
                        release_date: parse_date(row.release_date, "albums.csv", index + 2)?,
                        language: row.language.filter(|value| !value.trim().is_empty()),
                        version: row.version.filter(|value| !value.trim().is_empty()),
                    };
                    count += diesel::insert_into(albums::table).values(&insert).execute(connection)?;
                }
                Some(count)
            }
            None => None,
        };
        summary.record("albums.csv", album_count);

        // Relationship tables after every referenced parent table exists.
        import_plain!("group_companies.csv", GroupCompanyRow, group_companies::table);
        import_plain!("group_labels.csv", GroupLabelRow, group_labels::table);
        import_plain!("project_group_parents.csv", ProjectGroupParentRow, project_group_parents::table);
        import_plain!("idol_group_memberships.csv", IdolGroupMembershipRow, idol_group_memberships::table);
        import_plain!("idol_subunit_memberships.csv", IdolSubunitMembershipRow, idol_subunit_memberships::table);
        import_plain!("idol_project_group_memberships.csv", IdolProjectGroupMembershipRow, idol_project_group_memberships::table);
        import_plain!("idol_companies.csv", IdolCompanyRow, idol_companies::table);
        import_plain!("idol_labels.csv", IdolLabelRow, idol_labels::table);

        Ok(summary)
    })?;

    Ok(summary)
}
