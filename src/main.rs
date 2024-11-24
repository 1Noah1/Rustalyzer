mod error;

use axum::{
    routing::get, Router
};
use error::not_found_handler;



#[tokio::main]
async fn main() {
    let  app = Router::new()
    .route("/analyze", get(get_analyze).post(post_analyze))
    .route("/person/warning", get(person_warning))
    .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
 



// get measurement analysis
async fn get_analyze() {}

// analyze measurement
async fn post_analyze() {}

// should return a warning based of a person name
async fn person_warning() -> &'static str{
    "person-warning"
}

