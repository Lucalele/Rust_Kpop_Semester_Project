pub mod album;
pub mod companies;
pub mod database;
pub mod groups;
pub mod idol;
pub mod schema;

fn main() {
    let _connection = database::establish_connection();

    println!("Connected to database.");
}