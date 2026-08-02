use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::sqlite::SqliteConnection;

use crate::album::Album;

/// Every field is optional.
///
/// A field set to `None` is ignored.
/// When several fields are provided, they are combined with AND.
#[derive(Debug, Default)]
pub struct RandomizerFilters {
    // Direct album filters
    pub title: Option<String>, //goes unused because I realized that made no sense but I fear deleting it
    pub artist_id: Option<i32>,
    pub artist_type: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub language: Option<String>,
    pub version: Option<String>, //goes unused because I realized that made no sense but I fear deleting it

    // Artist and relationship filters
    pub artist_name: Option<String>,
    pub member_name: Option<String>,
    pub company_name: Option<String>,
    pub label_name: Option<String>,

    // Gender filters
    pub artist_gender: Option<String>,
    pub member_gender: Option<String>,
}

/// A private structure used to read Album rows from raw SQL.
#[derive(Debug, QueryableByName)]
struct AlbumRow {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    album_id: i32,

    #[diesel(sql_type = Text)]
    title: String,

    #[diesel(sql_type = diesel::sql_types::Integer)]
    artist_id: i32,

    #[diesel(sql_type = Text)]
    artist_type: String,

    #[diesel(sql_type = Nullable<diesel::sql_types::Date>)]
    release_date: Option<NaiveDate>,

    #[diesel(sql_type = Nullable<Text>)]
    language: Option<String>,

    #[diesel(sql_type = Nullable<Text>)]
    version: Option<String>,
}

impl From<AlbumRow> for Album {
    fn from(row: AlbumRow) -> Self {
        Album {
            album_id: row.album_id,
            title: row.title,
            artist_id: row.artist_id,
            artist_type: row.artist_type,
            release_date: row.release_date,
            language: row.language,
            version: row.version,
        }
    }
}

/// Returns randomized albums matching all supplied filters up to `amount`.
pub fn random_matching(
    connection: &mut SqliteConnection,
    filters: &RandomizerFilters,
    amount: i64,
) -> QueryResult<Vec<Album>> {
    run_randomizer_query(connection, filters, Some(amount.max(0)))
}

/// Returns a random set of albums across the entire collection without filters.
pub fn random_album(connection: &mut SqliteConnection, amount: i64) -> QueryResult<Vec<Album>> {
    random_matching(connection, &RandomizerFilters::default(), amount)
}

fn run_randomizer_query(
    connection: &mut SqliteConnection,
    filters: &RandomizerFilters,
    limit: Option<i64>,
) -> QueryResult<Vec<Album>> {
    // Format dates to ISO strings for reliable SQLite string comparison
    let start_date_str = filters.start_date.map(|d| d.format("%Y-%m-%d").to_string());
    let end_date_str = filters.end_date.map(|d| d.format("%Y-%m-%d").to_string());

    // Use i64::MAX as default if no limit is specified (SQLite treats high values as "no limit")
    let limit_val = limit.unwrap_or(i64::MAX);

    let rows = sql_query(
        r#"
        WITH filter_values AS (
            SELECT
                ? AS title_filter, 
                ? AS artist_id_filter,
                ? AS artist_type_filter,
                ? AS start_date_filter,
                ? AS end_date_filter,
                ? AS language_filter,
                ? AS version_filter,
                ? AS artist_name_filter,
                ? AS member_name_filter,
                ? AS company_name_filter,
                ? AS label_name_filter,
                ? AS artist_gender_filter,
                ? AS member_gender_filter
        )
        SELECT DISTINCT
            a.album_id,
            a.title,
            a.artist_id,
            a.artist_type,
            a.release_date,
            a.language,
            a.version
        FROM albums AS a
        CROSS JOIN filter_values AS f
        WHERE
            /* Direct album filters */
            (
                f.title_filter IS NULL
                OR a.title LIKE '%' || f.title_filter || '%'
            )
            AND (
                f.artist_id_filter IS NULL
                OR a.artist_id = f.artist_id_filter
            )
            AND (
                f.artist_type_filter IS NULL
                OR a.artist_type = f.artist_type_filter
            )
            AND (
                f.start_date_filter IS NULL
                OR a.release_date >= f.start_date_filter
            )
            AND (
                f.end_date_filter IS NULL
                OR a.release_date <= f.end_date_filter
            )
            AND (
                f.language_filter IS NULL
                OR a.language LIKE '%' || f.language_filter || '%'
            )
            AND (
                f.version_filter IS NULL
                OR a.version LIKE '%' || f.version_filter || '%'
            )

            /* Artist-name filter */
            AND (
                f.artist_name_filter IS NULL

                OR (
                    a.artist_type = 'Group'
                    AND EXISTS (
                        SELECT 1
                        FROM idol_groups AS g
                        WHERE g.group_id = a.artist_id
                          AND g.group_name LIKE '%' || f.artist_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'Subunit'
                    AND EXISTS (
                        SELECT 1
                        FROM subunits AS s
                        WHERE s.subunit_id = a.artist_id
                          AND s.subunit_name LIKE '%' || f.artist_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'ProjectGroup'
                    AND EXISTS (
                        SELECT 1
                        FROM project_groups AS pg
                        WHERE pg.project_group_id = a.artist_id
                          AND pg.project_group_name LIKE '%' || f.artist_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'Soloist'
                    AND EXISTS (
                        SELECT 1
                        FROM idol_names AS names
                        WHERE names.idol_id = a.artist_id
                          AND names.name LIKE '%' || f.artist_name_filter || '%'
                    )
                )
            )

            /* Member-name filter */
            AND (
                f.member_name_filter IS NULL

                OR EXISTS (
                    SELECT 1
                    FROM idols AS i
                    INNER JOIN idol_names AS names
                        ON names.idol_id = i.idol_id
                    WHERE
                        names.name LIKE '%' || f.member_name_filter || '%'

                        AND (
                            (
                                a.artist_type = 'Soloist'
                                AND a.artist_id = i.idol_id
                            )

                            OR (
                                a.artist_type = 'Group'
                                AND EXISTS (
                                    SELECT 1
                                    FROM idol_group_memberships AS igm
                                    WHERE igm.idol_id = i.idol_id
                                      AND igm.group_id = a.artist_id
                                )
                            )

                            OR (
                                a.artist_type = 'Subunit'
                                AND EXISTS (
                                    SELECT 1
                                    FROM idol_subunit_memberships AS ism
                                    WHERE ism.idol_id = i.idol_id
                                      AND ism.subunit_id = a.artist_id
                                )
                            )

                            OR (
                                a.artist_type = 'ProjectGroup'
                                AND EXISTS (
                                    SELECT 1
                                    FROM idol_project_group_memberships AS ipgm
                                    WHERE ipgm.idol_id = i.idol_id
                                      AND ipgm.project_group_id = a.artist_id
                                )
                            )
                        )
                )
            )

            /* Company filter */
            AND (
                f.company_name_filter IS NULL

                OR (
                    a.artist_type = 'Group'
                    AND EXISTS (
                        SELECT 1
                        FROM group_companies AS gc
                        INNER JOIN companies AS c ON c.company_id = gc.company_id
                        WHERE gc.group_id = a.artist_id
                          AND c.company_name LIKE '%' || f.company_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'Subunit'
                    AND EXISTS (
                        SELECT 1
                        FROM subunits AS s
                        INNER JOIN group_companies AS gc ON gc.group_id = s.parent_group_id
                        INNER JOIN companies AS c ON c.company_id = gc.company_id
                        WHERE s.subunit_id = a.artist_id
                          AND c.company_name LIKE '%' || f.company_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'ProjectGroup'
                    AND EXISTS (
                        SELECT 1
                        FROM project_group_parents AS pgp
                        INNER JOIN group_companies AS gc ON gc.group_id = pgp.parent_group_id
                        INNER JOIN companies AS c ON c.company_id = gc.company_id
                        WHERE pgp.project_group_id = a.artist_id
                          AND c.company_name LIKE '%' || f.company_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'Soloist'
                    AND EXISTS (
                        SELECT 1
                        FROM idol_companies AS ic
                        INNER JOIN companies AS c ON c.company_id = ic.company_id
                        WHERE ic.idol_id = a.artist_id
                          AND c.company_name LIKE '%' || f.company_name_filter || '%'
                    )
                )
            )

            /* Label filter */
            AND (
                f.label_name_filter IS NULL

                OR (
                    a.artist_type = 'Group'
                    AND EXISTS (
                        SELECT 1
                        FROM group_labels AS gl
                        INNER JOIN labels AS l ON l.label_id = gl.label_id
                        WHERE gl.group_id = a.artist_id
                          AND l.label_name LIKE '%' || f.label_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'Subunit'
                    AND EXISTS (
                        SELECT 1
                        FROM subunits AS s
                        INNER JOIN group_labels AS gl ON gl.group_id = s.parent_group_id
                        INNER JOIN labels AS l ON l.label_id = gl.label_id
                        WHERE s.subunit_id = a.artist_id
                          AND l.label_name LIKE '%' || f.label_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'ProjectGroup'
                    AND EXISTS (
                        SELECT 1
                        FROM project_group_parents AS pgp
                        INNER JOIN group_labels AS gl ON gl.group_id = pgp.parent_group_id
                        INNER JOIN labels AS l ON l.label_id = gl.label_id
                        WHERE pgp.project_group_id = a.artist_id
                          AND l.label_name LIKE '%' || f.label_name_filter || '%'
                    )
                )

                OR (
                    a.artist_type = 'Soloist'
                    AND EXISTS (
                        SELECT 1
                        FROM idol_labels AS il
                        INNER JOIN labels AS l ON l.label_id = il.label_id
                        WHERE il.idol_id = a.artist_id
                          AND l.label_name LIKE '%' || f.label_name_filter || '%'
                    )
                )
            )

            /* Artist-gender filter */
            AND (
                f.artist_gender_filter IS NULL

                OR (
                    a.artist_type = 'Group'
                    AND EXISTS (
                        SELECT 1
                        FROM idol_groups AS g
                        WHERE g.group_id = a.artist_id
                          AND g.gender = f.artist_gender_filter
                    )
                )

                OR (
                    a.artist_type = 'Subunit'
                    AND EXISTS (
                        SELECT 1
                        FROM subunits AS s
                        WHERE s.subunit_id = a.artist_id
                          AND s.gender = f.artist_gender_filter
                    )
                )

                OR (
                    a.artist_type = 'ProjectGroup'
                    AND EXISTS (
                        SELECT 1
                        FROM project_groups AS pg
                        WHERE pg.project_group_id = a.artist_id
                          AND pg.gender = f.artist_gender_filter
                    )
                )

                OR (
                    a.artist_type = 'Soloist'
                    AND EXISTS (
                        SELECT 1
                        FROM idols AS i
                        WHERE i.idol_id = a.artist_id
                          AND i.idol_gender = f.artist_gender_filter
                    )
                )
            )

            /* Member-gender filter */
            AND (
                f.member_gender_filter IS NULL

                OR EXISTS (
                    SELECT 1
                    FROM idols AS i
                    WHERE
                        i.idol_gender = f.member_gender_filter

                        AND (
                            (
                                a.artist_type = 'Soloist'
                                AND a.artist_id = i.idol_id
                            )

                            OR (
                                a.artist_type = 'Group'
                                AND EXISTS (
                                    SELECT 1
                                    FROM idol_group_memberships AS igm
                                    WHERE igm.idol_id = i.idol_id
                                      AND igm.group_id = a.artist_id
                                )
                            )

                            OR (
                                a.artist_type = 'Subunit'
                                AND EXISTS (
                                    SELECT 1
                                    FROM idol_subunit_memberships AS ism
                                    WHERE ism.idol_id = i.idol_id
                                      AND ism.subunit_id = a.artist_id
                                )
                            )

                            OR (
                                a.artist_type = 'ProjectGroup'
                                AND EXISTS (
                                    SELECT 1
                                    FROM idol_project_group_memberships AS ipgm
                                    WHERE ipgm.idol_id = i.idol_id
                                      AND ipgm.project_group_id = a.artist_id
                                )
                            )
                        )
                )
            )

        ORDER BY RANDOM()
        LIMIT ?
        "#,
    )
    .bind::<Nullable<Text>, _>(filters.title.clone())
    .bind::<Nullable<diesel::sql_types::Integer>, _>(filters.artist_id)
    .bind::<Nullable<Text>, _>(filters.artist_type.clone())
    .bind::<Nullable<Text>, _>(start_date_str)
    .bind::<Nullable<Text>, _>(end_date_str)
    .bind::<Nullable<Text>, _>(filters.language.clone())
    .bind::<Nullable<Text>, _>(filters.version.clone())
    .bind::<Nullable<Text>, _>(filters.artist_name.clone())
    .bind::<Nullable<Text>, _>(filters.member_name.clone())
    .bind::<Nullable<Text>, _>(filters.company_name.clone())
    .bind::<Nullable<Text>, _>(filters.label_name.clone())
    .bind::<Nullable<Text>, _>(filters.artist_gender.clone())
    .bind::<Nullable<Text>, _>(filters.member_gender.clone())
    .bind::<BigInt, _>(limit_val)
    .load::<AlbumRow>(connection)?;

    Ok(rows.into_iter().map(Album::from).collect())
}
