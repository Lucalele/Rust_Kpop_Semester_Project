use chrono::NaiveDate;
use diesel::prelude::*;

use crate::schema::albums;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = albums)]
#[diesel(primary_key(album_id))]
pub struct Album {
    pub album_id: i32,
    pub title: String,
    pub artist_id: i32,
    pub artist_type: String,
    pub release_date: Option<NaiveDate>,
    pub language: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = albums)]
pub struct NewAlbum<'a> {
    pub title: &'a str,
    pub artist_id: i32,
    pub artist_type: &'a str,
    pub release_date: Option<NaiveDate>,
    pub language: Option<&'a str>,
    pub version: Option<&'a str>,
}

pub fn insert_album(
    connection: &mut SqliteConnection,
    title: &str,
    artist_id: i32,
    artist_type: &str,
    release_date: Option<NaiveDate>,
    language: Option<&str>,
    version: Option<&str>,
) -> QueryResult<usize> {
    let new_album = NewAlbum {
        title,
        artist_id,
        artist_type,
        release_date,
        language,
        version,
    };

    diesel::insert_into(albums::table)
        .values(&new_album)
        .on_conflict((albums::title, albums::artist_id, albums::version))
        .do_nothing()
        .execute(connection)
}

pub fn load_albums(connection: &mut SqliteConnection) -> QueryResult<Vec<Album>> {
    albums::table
        .select(Album::as_select())
        .order(albums::album_id.asc())
        .load(connection)
}
