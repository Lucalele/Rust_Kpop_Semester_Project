use Rust_Kpop_Semester_Project::album::load_albums;
use Rust_Kpop_Semester_Project::companies::{load_companies, load_labels};
use Rust_Kpop_Semester_Project::database;
use Rust_Kpop_Semester_Project::groups::{
    load_groups,
    load_project_groups,
    load_subunits,
};
use Rust_Kpop_Semester_Project::idol::{load_idol_names, load_idols};
use Rust_Kpop_Semester_Project::importer;

fn main() {
    let mut connection = database::establish_selected_connection(0);

    importer::import_database_zero(
        &mut connection,
        "album.txt",
    )
    .unwrap();

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