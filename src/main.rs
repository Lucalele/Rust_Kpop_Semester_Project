use Rust_Kpop_Semester_Project::database;
use Rust_Kpop_Semester_Project::importer;

fn main() {
    let mut connection = database::establish_selected_connection(0);

    importer::import_database_zero(
        &mut connection,
        "album.txt",
    )
    .unwrap();
}