use diesel::prelude::*;
use crate::database;
use crate::importer;

use crate::loaders::{
    load_companies,
    load_labels,
    load_groups,
    load_subunits,
    load_project_groups,
    load_idols,
    load_idol_names,
    load_albums,
};

fn main() {
    // Connect to DB0 (main database)
    let mut connection = database::establish_selected_connection(0);

    // Initialize tape deck databases DB1–DB7
    for i in 1..=7 {
        let mut tape_conn = database::establish_selected_connection(i);
        database::initialize_tape_deck(&mut tape_conn);
    }

    // Import data into DB0
    importer::import_database_zero(
        &mut connection,
        "album.txt",
    ).expect("Failed to import DB0");

    // Print everything from DB0
    println!("Companies:");
    println!("{:#?}", load_companies(&mut connection).unwrap());

    println!("Labels:");
    println!("{:#?}", load_labels(&mut connection).unwrap());

    println!("Groups:");
    println!("{:#?}", load_groups(&mut connection).unwrap());

    println!("Subunits:");
    println!("{:#?}", load_subunits(&mut connection).unwrap());

    println!("Project Groups:");
    println!("{:#?}", load_project_groups(&mut connection).unwrap());

    println!("Idols:");
    println!("{:#?}", load_idols(&mut connection).unwrap());

    println!("Idol Names:");
    println!("{:#?}", load_idol_names(&mut connection).unwrap());

    println!("Albums:");
    println!("{:#?}", load_albums(&mut connection).unwrap());
}
