use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Internal imports
use Rust_Kpop_Semester_Project::importer::import_database_zero;
use Rust_Kpop_Semester_Project::randomizer::{RandomizerFilters, random_matching};

fn main() {
    println!("Starting K-Pop Archive...");

    // --- DATABASE 0 CHECK & CONNECTION ---
    let main_db = "src/database.sqlite";
    if !fs::metadata(main_db).is_ok() {
        eprintln!("Error: Main database not found at '{main_db}'");
        eprintln!("Please run: diesel migration run");
        return;
    }

    let mut conn = match SqliteConnection::establish(main_db) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to main database: {e}");
            eprintln!("Please run: diesel migration run");
            return;
        }
    };

    println!("Database 0 connection established.");

    // --- IMPORT ALBUM DATA ---
    let album_txt_path = "src/album.txt";
    if !Path::new(album_txt_path).exists() {
        eprintln!("Warning: '{album_txt_path}' not found! Skipping import step.");
    } else {
        println!("Importing data from '{album_txt_path}'...");
        match import_database_zero(&mut conn, album_txt_path) {
            Ok(_) => println!("Import successful!"),
            Err(e) => eprintln!("Error during data import: {e}"),
        }
    }

    // --- TRANSITION TO INTERACTIVE CLI MENU ---
    run_cli_menu(&mut conn);
}

// --- INTERACTIVE CLI MENU ---

fn run_cli_menu(conn: &mut SqliteConnection) {
    loop {
        println!("\n=========================================");
        println!("     K-POP ARCHIVE RANDOMIZER ENGINE     ");
        println!("=========================================");
        println!("1. Pick N Random Albums (using 1 Filter)");
        println!("2. Exit");
        print!("\nSelect an option (1-2): ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            break;
        }

        match choice.trim() {
            "1" => {
                // 1. Pick exactly one filter
                let filters = prompt_single_filter();

                // 2. Pick the amount of albums to retrieve
                let amount: i64 = prompt_str("\nHow many albums do you want to retrieve?: ")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);

                println!("\nSearching database for {} random match(es)...", amount);
                match random_matching(conn, &filters, amount) {
                    Ok(albums) => display_results(&albums),
                    Err(err) => eprintln!("Error fetching random albums: {err}"),
                }
            }
            "2" => {
                println!("Exiting K-Pop Archive. Goodbye!");
                break;
            }
            //As much as I hate erroring I did not want to make exiting the default or running the deafult as both could cause problems
            _ => println!("Invalid selection, please try again."), 
        }
    }
}

/// Prompt user to pick EXACTLY ONE filter category
fn prompt_single_filter() -> RandomizerFilters {
    println!("\n--- Choose ONE filter category ---"); //I did not want to layer filters as that would be a mess
    println!(" 1. Artist / Group Name");
    println!(" 2. Artist Type (Group/Soloist/Subunit/ProjectGroup)");
    println!(" 3. Idol Name");
    println!(" 4. Company Name");
    println!(" 5. Label Name");
    println!(" 6. Language");
    println!(" 7. Artist Gender (Female/Male/Co-ed)");
    println!(" 8. Idol Gender");
    println!(" 9. Released After Date (YYYY-MM-DD)");
    println!(" 10. Released Before Date (YYYY-MM-DD)");
    println!(" 11. No Filter (Random across all records)");

    let selection = prompt_str("\nSelect filter choice (1-11): ")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(11);

    let mut filters = RandomizerFilters::default();

    match selection {
        1 => filters.artist_name = prompt_str("Enter Artist Name: "),
        2 => filters.artist_type = prompt_str("Enter Artist Type: "), 
        3 => filters.member_name = prompt_str("Enter Member Name: "),
        4 => filters.company_name = prompt_str("Enter Company Name: "),
        5 => filters.label_name = prompt_str("Enter Label Name: "),
        6 => filters.language = prompt_str("Enter Language: "),
        7 => filters.artist_gender = prompt_str("Enter Artist Gender: "),
        8 => filters.member_gender = prompt_str("Enter Idol Gender: "),
        9 => {
            filters.start_date = prompt_str("Enter Start Date (YYYY-MM-DD): ")
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        }
        10 => {
            filters.end_date = prompt_str("Enter End Date (YYYY-MM-DD): ")
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        }
        //option 11 
        //or if anything other than a valid input is entered even if it's not an int
        //If you're putting something not completely stupid like no for no filter 
        //but I didn't want to check for no specifically
        //I don't want this thing to error out
        //So anything that is over 11 or not an int defaults here
        _ => println!("No filter selected."),
    }

    filters
}



fn prompt_str(label: &str) -> Option<String> {
    print!("{}", label);
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim().to_string();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    None
}

//only happens if a user enters 0 
//otherwise a garbage entry is treated as 1
//I don't like when programs give you an error instead of just powering through
fn display_results(albums: &[Rust_Kpop_Semester_Project::album::Album]) {
    if albums.is_empty() {
        println!("\n No matching albums found!");
        return;
    }

    println!("\n Found {} result(s):", albums.len());
    println!("{:-<65}", "");
    for (idx, album) in albums.iter().enumerate() {
        println!(
            "{:2}. {:<30} | {:<12} | {}",
            idx + 1,
            album.title,
            album.artist_type,
            album
                .release_date
                .map(|d| d.to_string())
                .unwrap_or_else(|| "N/A".into())
        );
    }
    println!("{:-<65}", "");
}
