use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Internal imports
use Rust_Kpop_Semester_Project::importer::import_database_zero;
use Rust_Kpop_Semester_Project::randomizer::{
    random_matching, RandomizerFilters,
};

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
            _ => println!("Invalid selection, please try again."),
        }
    }
}

/// Prompt user to pick EXACTLY ONE filter category
fn prompt_single_filter() -> RandomizerFilters {
    println!("\n--- Choose ONE filter category ---");
    println!(" 1. Album Title");
    println!(" 2. Artist / Group Name");
    println!(" 3. Artist Type (Group/Soloist/Subunit/ProjectGroup)");
    println!(" 4. Member Name");
    println!(" 5. Company Name");
    println!(" 6. Label Name");
    println!(" 7. Language");
    println!(" 8. Version");
    println!(" 9. Artist Gender (Female/Male/Co-ed)");
    println!("10. Member Gender (Female/Male)");
    println!("11. Released After Date (YYYY-MM-DD)");
    println!("12. Released Before Date (YYYY-MM-DD)");
    println!("13. No Filter (Random across all records)");

    let selection = prompt_str("\nSelect filter choice (1-13): ")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(13);

    let mut filters = RandomizerFilters::default();

    match selection {
        1 => filters.title = prompt_str("Enter Album Title: "),
        2 => filters.artist_name = prompt_str("Enter Artist/Group Name: "),
        3 => filters.artist_type = prompt_str("Enter Artist Type: "),
        4 => filters.member_name = prompt_str("Enter Member Name: "),
        5 => filters.company_name = prompt_str("Enter Company Name: "),
        6 => filters.label_name = prompt_str("Enter Label Name: "),
        7 => filters.language = prompt_str("Enter Language: "),
        8 => filters.version = prompt_str("Enter Version: "),
        9 => filters.artist_gender = prompt_str("Enter Artist Gender: "),
        10 => filters.member_gender = prompt_str("Enter Member Gender: "),
        11 => {
            filters.start_date = prompt_str("Enter Start Date (YYYY-MM-DD): ")
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        }
        12 => {
            filters.end_date = prompt_str("Enter End Date (YYYY-MM-DD): ")
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        }
        _ => println!("No filter selected."),
    }

    filters
}

// --- HELPER FUNCTIONS ---

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

fn display_results(albums: &[Rust_Kpop_Semester_Project::album::Album]) {
    if albums.is_empty() {
        println!("\n❌ No matching albums found!");
        return;
    }

    println!("\n🎲 Found {} result(s):", albums.len());
    println!("{:-<65}", "");
    for (idx, album) in albums.iter().enumerate() {
        println!(
            "{:2}. {:<30} | {:<12} | {}",
            idx + 1,
            album.title,
            album.artist_type,
            album.release_date.map(|d| d.to_string()).unwrap_or_else(|| "N/A".into())
        );
    }
    println!("{:-<65}", "");
}