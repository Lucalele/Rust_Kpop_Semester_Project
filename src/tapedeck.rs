use diesel::prelude::*;

use crate::schema_tapedecks::albums_alt;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = albums_alt)]
#[diesel(primary_key(album_id))]
pub struct AlbumAlt {
    pub album_id: i32,
    pub title: String,
    pub artist: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = albums_alt)]
pub struct NewAlbumAlt<'a> {
    pub title: &'a str,
    pub artist: &'a str,
}

pub fn insert_album_alt(
    connection: &mut SqliteConnection,
    title: &str,
    artist: &str,
) -> QueryResult<usize> {
    let new_album = NewAlbumAlt { title, artist };

    diesel::insert_into(albums_alt::table)
        .values(&new_album)
        .execute(connection)
}

pub fn load_albums_alt(
    connection: &mut SqliteConnection,
) -> QueryResult<Vec<AlbumAlt>> {
    albums_alt::table
        .select(AlbumAlt::as_select())
        .order(albums_alt::album_id.asc())
        .load(connection)
}