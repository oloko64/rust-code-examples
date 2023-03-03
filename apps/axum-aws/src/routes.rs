use crate::middlewares::validate_body;
use axum::{body, middleware, routing::get};
use axum::{response::IntoResponse, routing::post, Json, Router};
use hyper::{header, Method};
use serde::Deserialize;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::{
    cors::{self, CorsLayer},
    trace::TraceLayer,
};

use crate::errors::CustomResponse;

#[derive(Deserialize)]
pub struct BodyParams {
    name: String,
    age: u32,
}

pub fn get_app() -> Router {
    Router::new()
        .route("/name", post(print_body))
        // add this middleware to just the above routes, the order is bottom to top
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn(validate_body)),
        )
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            CorsLayer::new()
                // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin(cors::Any)
                .allow_methods([Method::GET, Method::POST])
                // It is required to add ".allow_headers([http::header::CONTENT_TYPE])"
                // See this issue https://github.com/tokio-rs/axum/issues/849
                .allow_headers([header::CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http())
}

async fn print_body(Json(body): Json<BodyParams>) -> Result<impl IntoResponse, CustomResponse> {
    if body.age < 18 {
        return Err(CustomResponse::BadRequest("You are too young!".to_string()));
    }

    Ok(Json(json!({
        "message": format!("Hello, {}!", body.name),
        "age": body.age,
    })))
}
