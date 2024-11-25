use axum::{
    routing::{get, post},
    Router,
};
use axum::http::{StatusCode, HeaderMap};
use axum::body::{Body, Bytes};
use axum::test::{TestClient, TestResponse};
use serde::{Deserialize, Serialize};
use hyper::header::CONTENT_TYPE;
use std::convert::Infallible;
use tower::ServiceExt;

#[derive(Serialize, Deserialize)]
struct AnalyzeRequest {
    data: String,
}

#[derive(Serialize, Deserialize)]
struct AnalyzeResponse {
    message: String,
}

// Your POST handler
async fn analyze_post(req: axum::Json<AnalyzeRequest>) -> axum::Json<AnalyzeResponse> {
    let data = &req.data;
    axum::Json(AnalyzeResponse {
        message: format!("Received data: {}", data),
    })
}

// Your main function
async fn main() {
    println!("this is the main function of the test");
    let app = Router::new()
        .route("/analyze", post(analyze_post))
        .fallback(|_| async { "Not Found".into() });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn test_analyze_post() {
    // Initialize the app and create a test client
    let app = Router::new()
        .route("/analyze", post(analyze_post));

    let client = TestClient::new(app);

    // Prepare the request payload
    let request_body = AnalyzeRequest {
        data: "test data".to_string(),
    };

    // Send the POST request to the `/analyze` endpoint
    let response: TestResponse = client
        .post("/analyze")
        .json(&request_body)
        .await
        .unwrap();

    // Ensure that the response status is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Ensure the response body contains the expected message
    let response_body: AnalyzeResponse = response.json().await.unwrap();
    assert_eq!(
        response_body.message,
        "Received data: test data"
    );
}

#[tokio::test]
async fn test_404_fallback() {
    // Initialize the app and create a test client
    let app = Router::new()
        .route("/analyze", post(analyze_post))
        .fallback(|_| async { "Not Found".into() });

    let client = TestClient::new(app);

    // Send a request to a non-existent route
    let response: TestResponse = client.get("/nonexistent").await.unwrap();

    // Ensure the response status is 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Check if the response body contains the fallback message
    let body = response.text().await.unwrap();
    assert_eq!(body, "Not Found");
}
