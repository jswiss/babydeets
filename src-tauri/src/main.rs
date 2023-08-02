// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axum::body::{Body, Bytes};
use axum::{
  routing::{get, post},
  http::StatusCode,
  response::IntoResponse,
  Json, Router};
  use axum::handler::Handler;

use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use tracing;
use tracing_subscriber;

// use tower_service::Service;
use std::str;

mod controllers;
mod db;

use crate::controllers::root::root;


#[tokio::main]
async fn main() {

init_server().await;

  tauri::Builder::default()
    .setup(|_app| {
      db::init();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
struct PrimitiveHttpRequest {
    path: String,
    method: String,
    body: Option<String>,
}


pub async fn init_server() {
  tracing_subscriber::fmt::init();
  let app = Router::new()
      .route("/", get(root));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("listening on {}", addr);
  axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
}
