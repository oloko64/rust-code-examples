use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

use crate::errors::CustomResponse;

#[derive(Deserialize)]
pub struct BodyParams {
    name: String,
    age: u32,
}

pub async fn print_body(Json(body): Json<BodyParams>) -> Result<impl IntoResponse, CustomResponse> {
    if body.age < 18 {
        return Err(CustomResponse::BadRequest("You are too young!".to_string()));
    }

    Ok(Json(json!({
        "message": format!("Hello, {}!", body.name),
        "age": body.age,
    })))
}
