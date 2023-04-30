use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::ApplicationState;

use super::{create_user, delete_user, get_user, get_users, update_user};

pub fn get_user_routes<T>() -> Router<T>
where
    T: ApplicationState + 'static,
{
    Router::new()
        .route("/users", post(create_user::<T>))
        .route("/users/:id", get(get_user::<T>))
        .route("/users", get(get_users::<T>))
        .route("/users/:id", put(update_user::<T>))
        .route("/users/:id", delete(delete_user::<T>))
}
