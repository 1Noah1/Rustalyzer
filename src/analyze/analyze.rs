use axum::{response::Response, Error};

mod PostMeasurement;

pub fn analyze_post(payload: Json<PostMeasurement>) -> Result<Response, Error>{
    println!("payload: {:?}", payload)
}

pub async fn analyze_get(){


}
