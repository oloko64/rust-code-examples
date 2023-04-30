use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use tracing::info;

use crate::{users::User, ApiError, ApplicationState};

pub mod routes;

pub async fn auth_user<T>(
    State(_pool): State<T>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, ApiError>
where
    T: ApplicationState,
{
    info!("Authenticating user: {}", user.get_email());
    let response = json!(
    {
        "message": "User authenticated",
        "user": user
    });

    Ok((StatusCode::OK, Json(response)))
}
