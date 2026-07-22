diesel::table! {
    albums_alt (album_id) {
        album_id -> Integer,
        title -> Text,
        artist -> Text,

    }
}
