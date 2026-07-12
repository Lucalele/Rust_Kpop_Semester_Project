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
    pub release_date: NaiveDate,
    pub language: String,
    pub version: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = albums)]
pub struct NewAlbum<'a> {
    pub title: &'a str,
    pub artist_id: i32,
    pub release_date: NaiveDate,
    pub language: &'a str,
    pub version: Option<&'a str>,
}