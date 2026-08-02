use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::NaiveDate;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::sqlite::SqliteConnection;

use crate::album::insert_album;
use crate::companies::{insert_company, insert_label};
use crate::groups::{
    insert_group, insert_group_company, insert_group_label, insert_project_group,
    insert_project_group_parent, insert_subunit,
};
use crate::idol::{
    insert_idol, insert_idol_company, insert_idol_group_membership, insert_idol_label,
    insert_idol_name, insert_idol_project_group_membership, insert_idol_subunit_membership,
};

//this is for dates
fn parse_date(value: &str) -> Option<NaiveDate> {
    if value.trim().is_empty() {
        None
    } else {
        NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
    }
}

//this is for soloists
fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None,
    }
}

/// Treat UNIQUE constraint violations as success (silent skip),
/// but still propagate all other errors.
/// This prevents the program from randomly deciding to not work because of data being weird
fn ignore_unique_violation(result: Result<usize, DieselError>) -> Result<(), Box<dyn Error>> {
    match result {
        Ok(_) => Ok(()),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            // Duplicate → silently ignored (A2 behavior)
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}


pub fn import_database_zero(
    connection: &mut SqliteConnection,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        //this is leftover from the old format of the text file but I'm too scared to remove this.
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();

        match parts[0] {
            "COMPANY" => {
                if parts.len() != 2 {
                    println!("Invalid COMPANY: {}", line);
                    continue;
                }

                // Silently ignores duplicates
                ignore_unique_violation(insert_company(connection, parts[1]))?;
                println!("Imported COMPANY {}", parts[1]);
            }

            "LABEL" => {
                if parts.len() != 3 {
                    println!("Invalid LABEL: {}", line);
                    continue;
                }

                let company_id: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_label(connection, parts[1], company_id))?;
                println!("Imported LABEL {}", parts[1]);
            }

            "GROUP" => {
                if parts.len() != 4 {
                    println!("Invalid GROUP: {}", line);
                    continue;
                }

                // Silently ignores duplicates
                ignore_unique_violation(insert_group(
                    connection,
                    parts[1],
                    parse_date(parts[2]),
                    parts[3],
                ))?;

                println!("Imported GROUP {}", parts[1]);
            }

            "SUBUNIT" => {
                if parts.len() != 5 {
                    println!("Invalid SUBUNIT: {}", line);
                    continue;
                }

                let parent_group: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_subunit(
                    connection,
                    parts[1],
                    parent_group,
                    parse_date(parts[3]),
                    parts[4],
                ))?;

                println!("Imported SUBUNIT {}", parts[1]);
            }

            "PROJECTGROUP" => {
                if parts.len() != 4 {
                    println!("Invalid PROJECTGROUP: {}", line);
                    continue;
                }

                // Silently ignores duplicates
                ignore_unique_violation(insert_project_group(
                    connection,
                    parts[1],
                    parse_date(parts[2]),
                    parts[3],
                ))?;

                println!("Imported PROJECTGROUP {}", parts[1]);
            }

            "PROJECTGROUPPARENT" => {
                if parts.len() != 3 {
                    println!("Invalid PROJECTGROUPPARENT: {}", line);
                    continue;
                }

                let project_group_id: i32 = parts[1].parse()?;
                let parent_group_id: i32 = parts[2].parse()?;

                //This is to avoid two project groups with the same parent group and ID of being an issue 
                //if the two Eunbis end up in the same project group
                //Which they genuinely could
                //Aka future proofing
                //But the program would just drop the second entry of Eunbi
                //Which the User shouldn't be entering them this way since that's not their stage names
                ignore_unique_violation(insert_project_group_parent(
                    connection,
                    project_group_id,
                    parent_group_id,
                ))?;

                println!(
                    "Linked PROJECTGROUP {} to parent GROUP {}",
                    project_group_id, parent_group_id
                );
            }

            "IDOL" => {
                if parts.len() != 3 {
                    println!("Invalid IDOL: {}", line);
                    continue;
                }

                // Silently ignores duplicates
                ignore_unique_violation(insert_idol(connection, parts[1], parse_bool(parts[2])))?;

                println!("Imported IDOL");
            }

            "IDOLNAME" => {
                if parts.len() != 3 {
                    println!("Invalid IDOLNAME: {}", line);
                    continue;
                }

                let idol_id: i32 = parts[1].parse()?;

                //This is actually important
                //In the group Gfriend both members have the actually name Eunbi
                //One goes by SinB and the other goes by Eunha
                //But they are both named Eunbi
                //The program will silently drop the second Eunbi
                ignore_unique_violation(insert_idol_name(connection, idol_id, parts[2]))?;
                println!("Added name {} to idol {}", parts[2], idol_id);
            }

            "IDOLGROUP" => {
                if parts.len() != 3 {
                    println!("Invalid IDOLGROUP: {}", line);
                    continue;
                }

                let idol_id: i32 = parts[1].parse()?;
                let group_id: i32 = parts[2].parse()?;

                //Again Eunbi Eunbi situation
                ignore_unique_violation(insert_idol_group_membership(
                    connection, idol_id, group_id,
                ))?;
                println!("Linked idol {} to group {}", idol_id, group_id);
            }

            "IDOLSUBUNIT" => {
                if parts.len() != 3 {
                    println!("Invalid IDOLSUBUNIT: {}", line);
                    continue;
                }

                let idol_id: i32 = parts[1].parse()?;
                let subunit_id: i32 = parts[2].parse()?;

                //Both Eunbis are in Viviz which some fans consider a subunit of Gfriend
                //So Eunbi dropping
                ignore_unique_violation(insert_idol_subunit_membership(
                    connection, idol_id, subunit_id,
                ))?;
                println!("Linked idol {} to subunit {}", idol_id, subunit_id);
            }

            "IDOLPROJECTGROUP" => {
                if parts.len() != 3 {
                    println!("Invalid IDOLPROJECTGROUP: {}", line);
                    continue;
                }

                let idol_id: i32 = parts[1].parse()?;
                let project_group_id: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_idol_project_group_membership(
                    connection,
                    idol_id,
                    project_group_id,
                ))?;

                println!(
                    "Linked idol {} to project group {}",
                    idol_id, project_group_id
                );
            }

            "IDOLCOMPANY" => {
                if parts.len() != 3 {
                    println!("Invalid IDOLCOMPANY: {}", line);
                    continue;
                }

                let idol_id: i32 = parts[1].parse()?;
                let company_id: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_idol_company(connection, idol_id, company_id))?;
                println!("Linked idol {} to company {}", idol_id, company_id);
            }

            "IDOLLABEL" => {
                if parts.len() != 3 {
                    println!("Invalid IDOLLABEL: {}", line);
                    continue;
                }

                let idol_id: i32 = parts[1].parse()?;
                let label_id: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_idol_label(connection, idol_id, label_id))?;
                println!("Linked idol {} to label {}", idol_id, label_id);
            }

            "GROUPCOMPANY" => {
                if parts.len() != 3 {
                    println!("Invalid GROUPCOMPANY: {}", line);
                    continue;
                }

                let group_id: i32 = parts[1].parse()?;
                let company_id: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_group_company(connection, group_id, company_id))?;
                println!("Linked group {} to company {}", group_id, company_id);
            }

            "GROUPLABEL" => {
                if parts.len() != 3 {
                    println!("Invalid GROUPLABEL: {}", line);
                    continue;
                }

                let group_id: i32 = parts[1].parse()?;
                let label_id: i32 = parts[2].parse()?;

                // Silently ignores duplicates
                ignore_unique_violation(insert_group_label(connection, group_id, label_id))?;
                println!("Linked group {} to label {}", group_id, label_id);
            }

            "ALBUM" => {
                if parts.len() != 7 {
                    println!("Invalid ALBUM: {}", line);
                    continue;
                }

                let artist_id: i32 = parts[2].parse()?;

                let language = if parts[5].trim().is_empty() {
                    None
                } else {
                    Some(parts[5])
                };

                let version = if parts[6].trim().is_empty() {
                    None
                } else {
                    Some(parts[6])
                };

                //this is handles two different versions of the same album by dropping them
                //if a user has multiple of the exact same version of the same album
                ignore_unique_violation(insert_album(
                    connection,
                    parts[1],
                    artist_id,
                    parts[3],
                    parse_date(parts[4]),
                    language,
                    version,
                ))?;

                println!("Imported ALBUM {}", parts[1]);
            }

            _ => {
                println!("Unknown command: {}", parts[0]);
            }
        }
    }

    Ok(())
}
