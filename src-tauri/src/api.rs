use serde::{Serialize, Deserialize};
use axum::{response::{Html, Response}, routing::get, Router, Error, http};
use axum::body::{Body, Bytes};

// use tower_service::Service;
use std::str;
use axum::handler::Handler;

#[derive(Serialize, Deserialize, Debug)]
struct PrimitiveHttpRequest {
    path: String,
    method: String,
    body: Option<String>,
}

// fn create_axum_router() -> Router {
  // let mut router: Router = Router::new()
  //   .route("/", get)
// }

pub async fn root() -> &'static str {
  "Sup, World!"
}
