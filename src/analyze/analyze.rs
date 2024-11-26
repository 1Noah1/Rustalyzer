use axum::{body::Body, response::Response};
use hyper::StatusCode;

use crate::model::model::Measurement;

pub async fn analyze_measurement(measurement: Measurement) -> Response<Body>{



}

