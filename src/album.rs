use chrono::NaiveDate;
use diesel::prelude::*;

use crate::schema::{albums, albums_alt};

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
    pub album_id: i32,
    pub title: &'a str,
    pub artist: &'a str,
}
