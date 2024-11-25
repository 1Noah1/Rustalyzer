use axum::{response::Response, Error, Json};
use hyper::StatusCode;
use serde_json::json;

use crate::model::PostMeasurement;

pub fn analyze_post(payload: Json<PostMeasurement>) -> Result<Response, Error>{
    println!("payload: {:?}", payload);
    axum::body::Body::new(Json(json!({"message": "success"}))
    Ok(Response::new(())), StatusCode::CREATED)
}

pub async fn analyze_get(){


}
