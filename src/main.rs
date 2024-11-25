mod error;
mod model;
mod analyze;

use axum::{
    routing::{get, post}, Router
};
use error::not_found_handler;
use analyze::analyze_post;

#[tokio::main]
async fn main() {
    let  app = Router::new()
    .route("/analyze", post(analyze_post))
    .route("/person/warning", get(person_warning))
    .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
 




// should return a warning based of a person name
async fn person_warning() -> &'static str{
    "person-warning"
}

