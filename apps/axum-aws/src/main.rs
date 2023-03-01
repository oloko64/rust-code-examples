use axum::{
    body::{BoxBody, self, Full, Bytes},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use hyper::{Request, StatusCode};
use serde::Deserialize;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{ServiceBuilderExt, trace::TraceLayer};
use tracing::{debug, info};

#[derive(Deserialize)]
struct BodyParams {
    name: String,
    age: u32,
}

enum CustomErrors {
    BadRequest(String),
    // InternalServerError(String),
}

impl IntoResponse for CustomErrors {
    fn into_response(self) -> Response {
        match self {
            CustomErrors::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "message": message,
                })),
            )
                .into_response(),
            // CustomErrors::InternalServerError(message) => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     Json(json!({
            //         "message": message,
            //     })),
            // )
            //     .into_response(),
        }
    }
}

async fn print_body(Json(body): Json<BodyParams>) -> Result<impl IntoResponse, CustomErrors> {
    if body.age < 18 {
        return Err(CustomErrors::BadRequest("You are too young!".to_string()));
    }

    Ok(Json(json!({
        "message": format!("Hello, {}!", body.name),
        "age": body.age,
    })))
}

// middleware that shows how to consume the request body upfront
async fn validate_body(
    request: Request<BoxBody>,
    next: Next<BoxBody>,
) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    // Check headers
    info!("headers: {:?}", request.headers());

    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request<BoxBody>) -> Result<Request<BoxBody>, Response> {
    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;

    // Do something with the body
    do_thing_with_request_body(bytes.clone());

    Ok(Request::from_parts(parts, body::boxed(Full::from(bytes))))
}

fn do_thing_with_request_body(bytes: Bytes) {
    info!(body = ?bytes);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/name", post(print_body))
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn(validate_body)),
        );

    debug!("Starting server on http://localhost:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
