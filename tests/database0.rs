use chrono::NaiveDate;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Text};
use diesel::sqlite::SqliteConnection;

use Rust_Kpop_Semester_Project::album::insert_album;
use Rust_Kpop_Semester_Project::companies::{
    insert_company,
    insert_label,
};
use Rust_Kpop_Semester_Project::database;
use Rust_Kpop_Semester_Project::groups::{
    insert_group,
    insert_group_company,
    insert_group_label,
    insert_project_group,
    insert_project_group_parent,
    insert_subunit,
};
use Rust_Kpop_Semester_Project::idol::{
    insert_idol,
    insert_idol_company,
    insert_idol_group_membership,
    insert_idol_label,
    insert_idol_name,
    insert_idol_project_group_membership,
    insert_idol_subunit_membership,
};
use Rust_Kpop_Semester_Project::randomizer::{
    random_matching,
    shuffle_matching,
    RandomizerFilters,
};

// -----------------------------------------------------------------------------
// Raw SQL result types
// -----------------------------------------------------------------------------

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

#[derive(Debug, QueryableByName)]
struct IdResult {
    #[diesel(sql_type = BigInt)]
    id: i64,
}

// -----------------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------------

/// Gets the automatically generated SQLite ID from the most recent insert.
fn last_insert_id(
    connection: &mut SqliteConnection,
) -> QueryResult<i32> {
    let result: IdResult =
        diesel::sql_query("SELECT last_insert_rowid() AS id;")
            .get_result(connection)?;

    i32::try_from(result.id).map_err(|error| {
        diesel::result::Error::DeserializationError(Box::new(error))
    })
}

/// Gets the number of rows currently stored in a table.
///
/// The table names used by this test are hard-coded below, so they are safe
/// to place into this SQL statement.
fn table_count(
    connection: &mut SqliteConnection,
    table_name: &str,
) -> QueryResult<i64> {
    let escaped_name = table_name.replace('"', "\"\"");

    let query = format!(
        "SELECT COUNT(*) AS count FROM \"{}\";",
        escaped_name
    );

    let result: CountResult =
        diesel::sql_query(query).get_result(connection)?;

    Ok(result.count)
}


// Complete database0 test
//Don't ask why this is one test
//I gave up on life 3 hours ago

#[test]
fn database_zero_is_valid_and_all_operations_work() {
    // Create database0 and run its migrations.
    database::initialize_all_dbz();

    let mut connection =
        database::establish_selected_connection(0);

    // =========================================================================
    // 1. Check SQLite configuration and integrity
    // =========================================================================

    let foreign_keys: ForeignKeysStatus =
        diesel::sql_query("PRAGMA foreign_keys;")
            .get_result(&mut connection)
            .expect("Could not check foreign-key status");

    assert_eq!(
        foreign_keys.foreign_keys,
        1,
        "Foreign-key enforcement is disabled"
    );

    let integrity: IntegrityResult =
        diesel::sql_query("PRAGMA integrity_check;")
            .get_result(&mut connection)
            .expect("Could not run SQLite integrity check");

    assert_eq!(
        integrity.integrity_check,
        "ok",
        "SQLite integrity check failed"
    );

    let violations: CountResult = diesel::sql_query(
        "
        SELECT COUNT(*) AS count
        FROM pragma_foreign_key_check;
        ",
    )
    .get_result(&mut connection)
    .expect("Could not run the foreign-key check");

    assert_eq!(
        violations.count,
        0,
        "database0 contains foreign-key violations"
    );

    // =========================================================================
    // 2. Check that every expected table exists
    // =========================================================================

    let tables: Vec<TableName> = diesel::sql_query(
        "
        SELECT name
        FROM sqlite_master
        WHERE type = 'table'
          AND name NOT LIKE 'sqlite_%'
          AND name != '__diesel_schema_migrations'
        ORDER BY name;
        ",
    )
    .load(&mut connection)
    .expect("Could not retrieve the database table list");

    let actual_table_names: Vec<&str> = tables
        .iter()
        .map(|table| table.name.as_str())
        .collect();

    let expected_tables = [
        "albums",
        "companies",
        "group_companies",
        "group_labels",
        "idol_companies",
        "idol_group_memberships",
        "idol_labels",
        "idol_names",
        "idol_project_group_memberships",
        "idol_subunit_memberships",
        "idols",
        "idol_groups",
        "labels",
        "project_group_parents",
        "project_groups",
        "subunits",
    ];

    for expected_table in expected_tables {
        assert!(
            actual_table_names.contains(&expected_table),
            "Expected table '{}' does not exist",
            expected_table
        );
    }

    // Verify that every application table can be queried.
    for table in &tables {
        let count = table_count(
            &mut connection,
            &table.name,
        )
        .unwrap_or_else(|error| {
            panic!(
                "Table '{}' could not be queried: {}",
                table.name,
                error
            )
        });

        println!("{}: {} row(s)", table.name, count);
    }

    // =========================================================================
    // 3. Test all inserts inside a rollback transaction
    //
    // Diesel rolls this transaction back after the test closure finishes.
    // The test records therefore do not permanently remain in database0.
    // =========================================================================

    connection.test_transaction::<_, diesel::result::Error, _>(
        |connection| {
            // Store counts so we can prove that each insert adds a row.
            let companies_before =
                table_count(connection, "companies")?;

            let labels_before =
                table_count(connection, "labels")?;

            let groups_before =
                table_count(connection, "idol_groups")?;

            let subunits_before =
                table_count(connection, "subunits")?;

            let project_groups_before =
                table_count(connection, "project_groups")?;

            let idols_before =
                table_count(connection, "idols")?;

            let idol_names_before =
                table_count(connection, "idol_names")?;

            let albums_before =
                table_count(connection, "albums")?;

            // -----------------------------------------------------------------
            // Company
            // -----------------------------------------------------------------

            insert_company(
                connection,
                "Database Zero Test Entertainment",
            )?;

            let company_id = last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "companies")?,
                companies_before + 1,
                "insert_company did not add a company"
            );

            // -----------------------------------------------------------------
            // Label
            // -----------------------------------------------------------------

            insert_label(
                connection,
                "Database Zero Test Label",
                company_id,
            )?;

            let label_id = last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "labels")?,
                labels_before + 1,
                "insert_label did not add a label"
            );

            // -----------------------------------------------------------------
            // Group
            // -----------------------------------------------------------------

            insert_group(
                connection,
                "Database Zero Test Group",
                NaiveDate::from_ymd_opt(2020, 1, 1),
                "Female",
            )?;

            let group_id = last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "idol_groups")?,
                groups_before + 1,
                "insert_group did not add a group"
            );

            // -----------------------------------------------------------------
            // Subunit
            // -----------------------------------------------------------------

            insert_subunit(
                connection,
                "Database Zero Test Subunit",
                group_id,
                NaiveDate::from_ymd_opt(2021, 1, 1),
                "Female",
            )?;

            let subunit_id = last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "subunits")?,
                subunits_before + 1,
                "insert_subunit did not add a subunit"
            );

            // -----------------------------------------------------------------
            // Project group
            // -----------------------------------------------------------------

            insert_project_group(
                connection,
                "Database Zero Test Project Group",
                NaiveDate::from_ymd_opt(2022, 1, 1),
                "CoEd",
            )?;

            let project_group_id =
                last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "project_groups")?,
                project_groups_before + 1,
                "insert_project_group did not add a project group"
            );

            // -----------------------------------------------------------------
            // Group relationships
            // -----------------------------------------------------------------

            insert_group_company(
                connection,
                group_id,
                company_id,
            )?;

            assert_eq!(
                table_count(connection, "group_companies")?,
                1,
                "insert_group_company did not add a relationship"
            );

            insert_group_label(
                connection,
                group_id,
                label_id,
            )?;

            assert_eq!(
                table_count(connection, "group_labels")?,
                1,
                "insert_group_label did not add a relationship"
            );

            insert_project_group_parent(
                connection,
                project_group_id,
                group_id,
            )?;

            assert_eq!(
                table_count(
                    connection,
                    "project_group_parents"
                )?,
                1,
                "insert_project_group_parent did not add a relationship"
            );

            // -----------------------------------------------------------------
            // Regular group member
            // -----------------------------------------------------------------

            insert_idol(
                connection,
                "Female",
                Some(false),
            )?;

            let member_id = last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "idols")?,
                idols_before + 1,
                "insert_idol did not add the first idol"
            );

            insert_idol_name(
                connection,
                member_id,
                "Database Zero Test Member",
            )?;

            assert_eq!(
                table_count(connection, "idol_names")?,
                idol_names_before + 1,
                "insert_idol_name did not add the first name"
            );

            // -----------------------------------------------------------------
            // Soloist
            // -----------------------------------------------------------------

            insert_idol(
                connection,
                "Male",
                Some(true),
            )?;

            let soloist_id = last_insert_id(connection)?;

            assert_eq!(
                table_count(connection, "idols")?,
                idols_before + 2,
                "insert_idol did not add the soloist"
            );

            insert_idol_name(
                connection,
                soloist_id,
                "Database Zero Test Soloist",
            )?;

            assert_eq!(
                table_count(connection, "idol_names")?,
                idol_names_before + 2,
                "insert_idol_name did not add the soloist name"
            );

            // -----------------------------------------------------------------
            // Idol memberships
            // -----------------------------------------------------------------

            insert_idol_group_membership(
                connection,
                member_id,
                group_id,
            )?;

            assert_eq!(
                table_count(
                    connection,
                    "idol_group_memberships"
                )?,
                1,
                "Group membership was not inserted"
            );

            insert_idol_subunit_membership(
                connection,
                member_id,
                subunit_id,
            )?;

            assert_eq!(
                table_count(
                    connection,
                    "idol_subunit_memberships"
                )?,
                1,
                "Subunit membership was not inserted"
            );

            insert_idol_project_group_membership(
                connection,
                member_id,
                project_group_id,
            )?;

            assert_eq!(
                table_count(
                    connection,
                    "idol_project_group_memberships"
                )?,
                1,
                "Project-group membership was not inserted"
            );

            // -----------------------------------------------------------------
            // Soloist company and label relationships
            // -----------------------------------------------------------------

            insert_idol_company(
                connection,
                soloist_id,
                company_id,
            )?;

            assert_eq!(
                table_count(connection, "idol_companies")?,
                1,
                "Idol/company relationship was not inserted"
            );

            insert_idol_label(
                connection,
                soloist_id,
                label_id,
            )?;

            assert_eq!(
                table_count(connection, "idol_labels")?,
                1,
                "Idol/label relationship was not inserted"
            );

            // -----------------------------------------------------------------
            // Albums for every artist type
            // -----------------------------------------------------------------

            insert_album(
                connection,
                "Database Zero Group Album",
                group_id,
                "Group",
                NaiveDate::from_ymd_opt(2023, 1, 1),
                Some("Korean"),
                Some("Standard"),
            )?;

            insert_album(
                connection,
                "Database Zero Subunit Album",
                subunit_id,
                "Subunit",
                NaiveDate::from_ymd_opt(2023, 6, 1),
                Some("Korean"),
                Some("Limited"),
            )?;

            insert_album(
                connection,
                "Database Zero Project Album",
                project_group_id,
                "ProjectGroup",
                NaiveDate::from_ymd_opt(2024, 1, 1),
                Some("Japanese"),
                Some("Standard"),
            )?;

            insert_album(
                connection,
                "Database Zero Solo Album",
                soloist_id,
                "Soloist",
                NaiveDate::from_ymd_opt(2025, 1, 1),
                Some("English"),
                Some("Digital"),
            )?;

            assert_eq!(
                table_count(connection, "albums")?,
                albums_before + 4,
                "The four test albums were not inserted"
            );

            // =========================================================================
            // 4. Run another integrity check after all inserts
            // =========================================================================

            let violations_after_inserts: CountResult =
                diesel::sql_query(
                    "
                    SELECT COUNT(*) AS count
                    FROM pragma_foreign_key_check;
                    ",
                )
                .get_result(connection)?;

            assert_eq!(
                violations_after_inserts.count,
                0,
                "The insert functions created foreign-key violations"
            );

            // =========================================================================
            // 5. Test random_matching
            // =========================================================================

            let no_filters = RandomizerFilters::default();

            let random_results = random_matching(
                connection,
                &no_filters,
                2,
            )?;

            assert_eq!(
                random_results.len(),
                2,
                "random_matching should return the requested two albums"
            );

            // -----------------------------------------------------------------
            // Language filter
            // -----------------------------------------------------------------

            let korean_results = random_matching(
                connection,
                &RandomizerFilters {
                    language: Some("Korean".to_string()),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                korean_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "The Korean filter did not return the group album"
            );

            assert!(
                korean_results.iter().any(|album| {
                    album.title == "Database Zero Subunit Album"
                }),
                "The Korean filter did not return the subunit album"
            );

            assert!(
                korean_results.iter().all(|album| {
                    album.language.as_deref() == Some("Korean")
                }),
                "The Korean filter returned a non-Korean album"
            );

            // -----------------------------------------------------------------
            // Version filter
            // -----------------------------------------------------------------

            let limited_results = random_matching(
                connection,
                &RandomizerFilters {
                    version: Some("Limited".to_string()),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                limited_results.iter().any(|album| {
                    album.title
                        == "Database Zero Subunit Album"
                }),
                "The version filter did not return the limited album"
            );

            assert!(
                limited_results.iter().all(|album| {
                    album.version.as_deref() == Some("Limited")
                }),
                "The version filter returned the wrong version"
            );

            // -----------------------------------------------------------------
            // Artist type filter
            // -----------------------------------------------------------------

            let soloist_results = random_matching(
                connection,
                &RandomizerFilters {
                    artist_type: Some("Soloist".to_string()),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                soloist_results.iter().any(|album| {
                    album.title == "Database Zero Solo Album"
                }),
                "The Soloist filter did not return the solo album"
            );

            assert!(
                soloist_results.iter().all(|album| {
                    album.artist_type == "Soloist"
                }),
                "The Soloist filter returned another artist type"
            );

            // -----------------------------------------------------------------
            // Artist-name filter
            // -----------------------------------------------------------------

            let group_name_results = random_matching(
                connection,
                &RandomizerFilters {
                    artist_name: Some(
                        "Database Zero Test Group".to_string(),
                    ),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                group_name_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "The artist-name filter did not return the group album"
            );

            // -----------------------------------------------------------------
            // Member-name filter
            // -----------------------------------------------------------------

            let member_results = random_matching(
                connection,
                &RandomizerFilters {
                    member_name: Some(
                        "Database Zero Test Member".to_string(),
                    ),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                member_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "Member filter did not find the member's group"
            );

            assert!(
                member_results.iter().any(|album| {
                    album.title
                        == "Database Zero Subunit Album"
                }),
                "Member filter did not find the member's subunit"
            );

            assert!(
                member_results.iter().any(|album| {
                    album.title
                        == "Database Zero Project Album"
                }),
                "Member filter did not find the member's project group"
            );

            // -----------------------------------------------------------------
            // Company filter
            // -----------------------------------------------------------------

            let company_results = random_matching(
                connection,
                &RandomizerFilters {
                    company_name: Some(
                        "Database Zero Test Entertainment"
                            .to_string(),
                    ),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                company_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "Company filter did not find the group album"
            );

            assert!(
                company_results.iter().any(|album| {
                    album.title == "Database Zero Solo Album"
                }),
                "Company filter did not find the solo album"
            );

            // -----------------------------------------------------------------
            // Label filter
            // -----------------------------------------------------------------

            let label_results = random_matching(
                connection,
                &RandomizerFilters {
                    label_name: Some(
                        "Database Zero Test Label".to_string(),
                    ),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                label_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "Label filter did not find the group album"
            );

            assert!(
                label_results.iter().any(|album| {
                    album.title == "Database Zero Solo Album"
                }),
                "Label filter did not find the solo album"
            );

            // -----------------------------------------------------------------
            // Artist gender filter
            // -----------------------------------------------------------------

            let coed_results = random_matching(
                connection,
                &RandomizerFilters {
                    artist_gender: Some("CoEd".to_string()),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                coed_results.iter().any(|album| {
                    album.title
                        == "Database Zero Project Album"
                }),
                "CoEd artist filter did not return the project album"
            );

            // -----------------------------------------------------------------
            // Member gender filter
            // -----------------------------------------------------------------

            let female_member_results = random_matching(
                connection,
                &RandomizerFilters {
                    member_gender: Some("Female".to_string()),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                female_member_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "Female-member filter did not find the group album"
            );

            // -----------------------------------------------------------------
            // Date filters
            // -----------------------------------------------------------------

            let date_results = random_matching(
                connection,
                &RandomizerFilters {
                    start_date: NaiveDate::from_ymd_opt(
                        2023, 1, 1,
                    ),
                    end_date: NaiveDate::from_ymd_opt(
                        2023, 12, 31,
                    ),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                date_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "Date filter did not return the 2023 group album"
            );

            assert!(
                date_results.iter().any(|album| {
                    album.title
                        == "Database Zero Subunit Album"
                }),
                "Date filter did not return the 2023 subunit album"
            );

            assert!(
                !date_results.iter().any(|album| {
                    album.title
                        == "Database Zero Project Album"
                }),
                "Date filter incorrectly returned the 2024 album"
            );

            assert!(
                !date_results.iter().any(|album| {
                    album.title == "Database Zero Solo Album"
                }),
                "Date filter incorrectly returned the 2025 album"
            );

            // -----------------------------------------------------------------
            // Impossible filter
            // -----------------------------------------------------------------

            let empty_results = random_matching(
                connection,
                &RandomizerFilters {
                    artist_name: Some(
                        "This Artist Does Not Exist".to_string(),
                    ),
                    ..Default::default()
                },
                100,
            )?;

            assert!(
                empty_results.is_empty(),
                "A nonexistent artist should return no albums"
            );

            // =========================================================================
            // 6. Test shuffle_matching
            // =========================================================================

            let shuffled_results = shuffle_matching(
                connection,
                &RandomizerFilters::default(),
            )?;

            assert!(
                shuffled_results.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "shuffle_matching omitted the group album"
            );

            assert!(
                shuffled_results.iter().any(|album| {
                    album.title
                        == "Database Zero Subunit Album"
                }),
                "shuffle_matching omitted the subunit album"
            );

            assert!(
                shuffled_results.iter().any(|album| {
                    album.title
                        == "Database Zero Project Album"
                }),
                "shuffle_matching omitted the project album"
            );

            assert!(
                shuffled_results.iter().any(|album| {
                    album.title == "Database Zero Solo Album"
                }),
                "shuffle_matching omitted the solo album"
            );

            let shuffled_korean = shuffle_matching(
                connection,
                &RandomizerFilters {
                    language: Some("Korean".to_string()),
                    ..Default::default()
                },
            )?;

            assert!(
                shuffled_korean.iter().any(|album| {
                    album.title == "Database Zero Group Album"
                }),
                "Filtered shuffle omitted the Korean group album"
            );

            assert!(
                shuffled_korean.iter().any(|album| {
                    album.title
                        == "Database Zero Subunit Album"
                }),
                "Filtered shuffle omitted the Korean subunit album"
            );

            assert!(
                shuffled_korean.iter().all(|album| {
                    album.language.as_deref() == Some("Korean")
                }),
                "Filtered shuffle returned a non-Korean album"
            );

            Ok(())
        },
    );
}