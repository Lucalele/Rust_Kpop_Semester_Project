use Rust_Kpop_Semester_Project::database::{self, establish_connection};

fn main() {
    establish_connection();
    database::initialize_all_tape_decks();

    println!("All seven tape decks are ready");

    database::initialize_all_dbz();

    println!("IT WORKED");
}
