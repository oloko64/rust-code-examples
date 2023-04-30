use axum::{routing::post, Router};

use crate::ApplicationState;

use super::auth_user;

pub fn get_auth_routes<T>() -> Router<T>
where
    T: ApplicationState + 'static,
{
    Router::new().route("/auth", post(auth_user::<T>))
}
