// This small schema is maintained manually because the seven tape-deck
// databases all share the same table layout. Running Diesel print-schema
// against the main archive should not overwrite this file.
//Supposedely
//Maybe don't try it

diesel::table! {
    albums_alt (album_id) {
        album_id -> Integer,
        title -> Text,
        artist -> Text,
    }
}
