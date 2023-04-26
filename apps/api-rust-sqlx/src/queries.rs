use sqlx::SqlitePool;

use crate::{ApiError, NewUser, User};

pub async fn new_user(conn_pool: &SqlitePool, user: NewUser) -> Result<String, ApiError> {
    sqlx::query!("INSERT INTO users (name, email) VALUES (?, ?)", user.name, user.email)
        .execute(conn_pool)
        .await
        .map_err(|err| ApiError {
            error: err.to_string(),
        })?;

    Ok(String::from("OK"))
}

pub async fn list_user(conn_pool: &SqlitePool, user_id: i64) -> Result<User, ApiError> {
    let user = sqlx::query!("SELECT * FROM users WHERE id = ?", user_id)
        .fetch_one(conn_pool)
        .await
        .map_err(|err| ApiError {
            error: err.to_string(),
        })?;

    let user = User {
        id: user.id,
        name: user.name,
        email: user.email,
    };

    Ok(user)
}

pub async fn list_users(conn_pool: &SqlitePool) -> Result<Vec<User>, ApiError> {
    let users = sqlx::query!("SELECT * FROM users")
        .fetch_all(conn_pool)
        .await
        .map_err(|err| ApiError {
            error: err.to_string(),
        })?;

    let users = users
        .into_iter()
        .map(|user| User {
            id: user.id,
            name: user.name,
            email: user.email,
        })
        .collect::<Vec<_>>();

    Ok(users)
}
