use ::time::OffsetDateTime;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;

use crate::{ApiError, ApplicationState};

use self::model::UserDatabase;

mod model;
pub mod routes;

pub async fn create_user<T>(
    State(pool): State<T>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, ApiError>
where
    T: ApplicationState,
{
    let hashed_password =
        hash(user.password, DEFAULT_COST).map_err(|err| ApiError::InternalServerError {
            error: err.to_string(),
        })?;
    let query = sqlx::query_as!(
        UserDatabase,
        "INSERT INTO users (name, email, password, active) VALUES ($1, $2, $3, $4) RETURNING id, name, email, password, active, created_at, updated_at",
        user.name,
        user.email,
        hashed_password,
        user.active
    )
    .fetch_one(pool.get_pg_pool_ref())
    .await.map_err(|err| {
        ApiError::InternalServerError {
            error: err.to_string(),
        }
    })?;

    let user = User {
        id: Some(query.id),
        name: query.name,
        email: query.email,
        password: query.password,
        active: query.active,
        created_at: query.created_at.to_string(),
        updated_at: query.updated_at.to_string(),
    };

    Ok((StatusCode::OK, Json(user)))
}

pub async fn get_user<T>(
    State(pool): State<T>,
    Path(params): Path<Params>,
) -> Result<impl IntoResponse, ApiError>
where
    T: ApplicationState,
{
    let query = sqlx::query_as!(
        UserDatabase,
        "SELECT id, name, email, password, active, created_at, updated_at FROM users WHERE id = $1",
        params.id
    )
    .fetch_one(pool.get_pg_pool_ref())
    .await
    .map_err(|err| ApiError::InternalServerError {
        error: err.to_string(),
    })?;

    let user = User {
        id: Some(query.id),
        name: query.name,
        email: query.email,
        password: query.password,
        active: query.active,
        created_at: query.created_at.to_string(),
        updated_at: query.updated_at.to_string(),
    };

    Ok((StatusCode::OK, Json(user)))
}

pub async fn get_users<T>(State(pool): State<T>) -> Result<impl IntoResponse, ApiError>
where
    T: ApplicationState,
{
    let query = sqlx::query_as!(
        UserDatabase,
        "SELECT id, name, email, password, active, created_at, updated_at FROM users"
    )
    .fetch_all(pool.get_pg_pool_ref())
    .await
    .map_err(|err| ApiError::InternalServerError {
        error: err.to_string(),
    })?;

    let users = query
        .into_iter()
        .map(|user| User {
            id: Some(user.id),
            name: user.name,
            email: user.email,
            password: user.password,
            active: user.active,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        })
        .collect::<Vec<User>>();

    Ok((StatusCode::OK, Json(users)))
}

pub async fn update_user<T>(
    State(pool): State<T>,
    Path(params): Path<Params>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, ApiError>
where
    T: ApplicationState,
{
    let hashed_password =
        hash(user.password, DEFAULT_COST).map_err(|err| ApiError::InternalServerError {
            error: err.to_string(),
        })?;
    let now = OffsetDateTime::now_utc();
    let current_time = PrimitiveDateTime::new(now.date(), now.time());
    let query = sqlx::query_as!(
        UserDatabase,
        "UPDATE users SET name = $1, email = $2, password = $3, active = $4, updated_at = $5 WHERE id = $6 RETURNING id, name, email, password, active, created_at, updated_at",
        user.name,
        user.email,
        hashed_password,
        user.active,
        current_time,
        params.id
    )
    .fetch_one(pool.get_pg_pool_ref())
    .await.map_err(|err| {
        ApiError::InternalServerError { error: err.to_string() }
    })?;

    let user = User {
        id: Some(query.id),
        name: query.name,
        email: query.email,
        password: query.password,
        active: query.active,
        created_at: query.created_at.to_string(),
        updated_at: query.updated_at.to_string(),
    };

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user<T>(
    State(pool): State<T>,
    Path(params): Path<Params>,
) -> Result<impl IntoResponse, ApiError>
where
    T: ApplicationState,
{
    sqlx::query!("DELETE FROM users WHERE id = $1", params.id)
        .execute(pool.get_pg_pool_ref())
        .await
        .map_err(|err| ApiError::InternalServerError {
            error: err.to_string(),
        })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,

    name: String,
    email: String,
    password: String,

    #[serde(skip_deserializing)]
    active: bool,

    #[serde(skip_deserializing)]
    created_at: String,

    #[serde(skip_deserializing)]
    updated_at: String,
}

impl User {
    pub fn get_email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Deserialize)]
pub struct Params {
    id: i32,
}
