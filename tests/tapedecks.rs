use diesel::Connection;
use diesel::sqlite::SqliteConnection;

use Rust_Kpop_Semester_Project::database::initialize_tape_deck;
use Rust_Kpop_Semester_Project::tapedeck::{insert_album_alt, load_albums_alt};

#[test]
fn inserts_and_loads_tape_deck_album() {
    let mut connection =
        SqliteConnection::establish(":memory:").expect("Could not create test database");

    initialize_tape_deck(&mut connection);

    let inserted = insert_album_alt(&mut connection, "Formula of Love", "TWICE")
        .expect("Could not insert album");

    assert_eq!(inserted, 1);

    let albums = load_albums_alt(&mut connection).expect("Could not load albums");

    assert_eq!(albums.len(), 1);
    assert_eq!(albums[0].title, "Formula of Love");
    assert_eq!(albums[0].artist, "TWICE");
}
