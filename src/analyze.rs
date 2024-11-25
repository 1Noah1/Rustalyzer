use axum::{body::Body, response::Response};
use hyper::StatusCode;

pub async fn analyze_post() -> Response<Body>{

    let body = Body::from("test");

    Response::builder()
        .status(StatusCode::CREATED)
        .header("x-foo", "custom header")
        .body(body)
        .unwrap()

}

