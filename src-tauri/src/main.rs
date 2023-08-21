// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use color_eyre::eyre::Result;

mod models;
mod schema;
mod db;
mod commands;
mod services;

use crate::db::db_init;
use crate::commands::baby_commands::*;

fn main() -> Result<()> {
  color_eyre::install()?;

  tauri::Builder::default()
    .setup(|_app| {
      db_init();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      create_baby,
      list_babies
    ])
    .plugin(tauri_plugin_sqlite_store::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    Ok(())
}
