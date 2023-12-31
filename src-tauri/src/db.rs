use std::fs;
use std::path::Path;
use std::env;

use dotenvy::dotenv;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn db_init() {
  if !db_file_exists() {
      create_db_file();
  }

  run_migrations();
}

pub fn establish_db_connection() -> SqliteConnection {
dotenv().ok();

let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&db_url)
  .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

fn run_migrations() {
println!("preparing to run migrations");
let mut connection = establish_db_connection();
connection.run_pending_migrations(MIGRATIONS).unwrap();
println!("ran migrations");
}

// Create the database file.
fn create_db_file() {
  let db_path = get_db_path();
  let db_dir = Path::new(&db_path).parent().unwrap();

  // If the parent directory does not exist, create it.
  if !db_dir.exists() {
      fs::create_dir_all(db_dir).unwrap();
  }

  // Create the database file.
  fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
  let db_path = get_db_path();
  Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
fn get_db_path() -> String {
  let home_dir = dirs::home_dir().unwrap();
  home_dir.to_str().unwrap().to_string() + "/.config/babydeets/database.sqlite"
}
