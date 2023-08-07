// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
mod models;
mod schema;
mod db;

use crate::db::db_init;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      db_init();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
