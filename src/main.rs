use Rust_Kpop_Semester_Project::database;

fn main() {
    database::initialize_all_tape_decks();

    println!("All seven tape decks are ready");

    database::initialize_all_dbz();

    println!("IT WORKED");
}