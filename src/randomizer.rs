//use diesel::dsl::frame::Groups;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel::sqlite::SqliteConnection;

use crate::album::Album;
//use crate::groups;
use crate::schema::albums;

pub fn random_album(connection: &mut SqliteConnection, amount: i64) -> QueryResult<Vec<Album>> {
    albums::table
        .select(Album::as_select())
        .order(sql::<Integer>("RANDOM()"))
        .limit(amount)
        .load(connection)
}

pub fn shuffle(connection: &mut SqliteConnection) -> QueryResult<Vec<Album>> {
    albums::table
        .select(Album::as_select())
        .order(sql::<Integer>("RANDOM()"))
        .load(connection)
}
