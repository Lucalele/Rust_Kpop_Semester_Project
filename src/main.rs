pub mod album;
pub mod companies;
pub mod database;
pub mod groups;
pub mod idol;
pub mod schema;
pub mod csv_import;

fn main() {
    database::initialize_all_tape_decks();

    println!("All seven tape decks are ready");
}