mod error;
mod analyze;
mod model;

use analyze::analyze::analyze_measurement;
use axum::{
    response::IntoResponse, routing::{get, post}, Json, Router, debug_handler
};
use error::not_found_handler;
use model::model::Measurement;
use serde_json::json;

#[tokio::main]
async fn main() {
    let  app = Router::new()
    .route("/patients", get(patients_get))
    .route("/analyze", post(analyze_post))

    .route("/warning", get(warnings_get))
    .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
 

async fn patients_get (){
    // return all patients
}

async fn warnings_get (){
    // return all warnings
}

#[debug_handler]
async fn analyze_post (Json(payload): Json<Measurement>) -> impl IntoResponse {
    let measurement = payload;
    println!("Received measurement: {:?}", measurement);

    analyze_measurement(measurement).await;
    let json_dummy = axum::Json(json!({"status": "ok"}));
    json_dummy.into_response()
}
