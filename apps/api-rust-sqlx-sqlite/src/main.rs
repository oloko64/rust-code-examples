mod queries;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use queries::{list_user, list_users, new_user};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or("debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = SqlitePool::connect(&database_url).await?;
    let app = Router::new()
        .route("/users", get(get_all_users))
        .route("/users/:id", get(get_user))
        .route("/add_user", post(add_user))
        .with_state(pool);

    info!("Starting server at port {}", 5000);

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_all_users(State(pool): State<SqlitePool>) -> Result<impl IntoResponse, ApiError> {
    let users = list_users(&pool).await.unwrap();

    Ok((StatusCode::OK, Json(users)))
}

async fn get_user(
    State(pool): State<SqlitePool>,
    Path(params): Path<Params>,
) -> Result<impl IntoResponse, ApiError> {
    let user = list_user(&pool, params.id).await.unwrap();

    Ok((StatusCode::OK, Json(user)))
}

async fn add_user(
    State(pool): State<SqlitePool>,
    Json(params): Json<NewUser>,
) -> Result<impl IntoResponse, ApiError> {
    let user = new_user(&pool, params).await?;

    Ok((StatusCode::OK, Json(user)))
}

#[derive(Debug, Serialize)]
pub struct User {
    id: i64,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Params {
    id: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(String::from("Internal Server Error")),
        )
            .into_response()
    }
}
