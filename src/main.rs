pub mod album;
pub mod companies;
pub mod database;
pub mod groups;
pub mod idol;
pub mod schema;
pub mod csv_import;
//mod csv_import;

fn main() {
    // 0 opens the special main database from .env
    let _connection = database::establish_selected_connection(0);

    println!("Connected to main database");


}