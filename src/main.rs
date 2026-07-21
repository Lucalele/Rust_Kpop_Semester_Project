pub mod album;
pub mod companies;
pub mod database;
pub mod groups;
pub mod idol;
pub mod schema;
pub mod csv_import;
pub mod randomizer;

fn main() {
    database::initialize_all_tape_decks();

    println!("All seven tape decks are ready");

    database::initialize_all_dbz();

    println!("IT WORKED");
}