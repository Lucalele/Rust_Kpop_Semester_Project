If a user encounters anything pertaining to database 1-7, it should be deleted immediately.

It should all be scrubed, but who knows.

It will corrupt the files if ran.

If diesel print-schema0 was ran on database 1-7,
diesel would change Interger NOT NULL to Nullable Intereger

Then if you cargo run

Something something u16 to u8 ascii corruption.

It even corrupted in the terminal.

Hence why all tests were also removed.

Diesel apparently is not meant to house two simultaneous schema
when the person writing the code can barely manage one.

In the event of repository corruption, reclone the repository.

The program can be run with cargo run. 

Do not run it with anything else except cargo run --release which does work. 

Just use cargo run.

There is a text file to populate the database.

It will do this automatically.

Do not mess with the file.

Unless you're Professor Kennedy, then do what you want at your own risk.

Add new data by following the code pattern in the importer. 

The importer.rs like database.rs is held together with duct tape and my will to live.

It is not incoherent logically, just very long. 


Dependencies
chrono = "0.4.45"
csv = "1.4.0" //unused but wanted for future when I keep working on this
diesel = { version = "2.3.10", features = ["sqlite", "chrono"] }
libsqlite3-sys = { version = "0.37", features = ["bundled"] }
dotenv = "0.15.0"
rand = "0.8" // 
serde = "1.0.229" 



