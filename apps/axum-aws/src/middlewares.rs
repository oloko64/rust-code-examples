use axum::{
    body::{self, BoxBody, Bytes, Full},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::{Request, StatusCode};
use tracing::info;

use crate::errors::CustomResponse;

// middleware that shows how to consume the request body upfront
pub async fn validate_body(
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
        .map_err(|err| CustomResponse::InternalServerError(err.to_string()).into_response())?;

    // Do something with the body, if it modifies the body, you can use `.clone()` to make a copy
    do_thing_with_request_body(&bytes)?;

    Ok(Request::from_parts(parts, body::boxed(Full::from(bytes))))
}

fn do_thing_with_request_body(bytes: &Bytes) -> Result<(), Response> {
    info!(body = ?bytes);
    if bytes.len() > 100 {
        // You can use a tuple to return a custom error
        return Err((StatusCode::BAD_REQUEST, "Body too large").into_response());
    }
    Ok(())
}
