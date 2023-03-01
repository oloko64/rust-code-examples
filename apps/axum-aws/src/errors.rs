use axum::response::Response;
use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

pub enum CustomResponse {
    BadRequest(String),
    InternalServerError(String),
}

impl IntoResponse for CustomResponse {
    fn into_response(self) -> Response {
        match self {
            CustomResponse::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": message,
                })),
            )
                .into_response(),
            CustomResponse::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": message,
                })),
            )
                .into_response(),
        }
    }
}
