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
