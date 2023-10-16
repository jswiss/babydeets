// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod models;
mod schema;
mod db;
mod commands;
mod services;
mod fns;

use crate::db::db_init;
use crate::commands::baby_commands::*;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      db_init();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      create_baby,
      list_babies,
      get_baby,
      update_baby,
      upload_baby_files
    ])
    .plugin(tauri_plugin_websocket::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  }
