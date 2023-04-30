mod auth;
mod users;

use axum::{
    response::{IntoResponse, Response},
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use crate::{auth::routes::get_auth_routes, users::routes::get_user_routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or("debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let state = AppState {
        pool: PgPool::connect(&database_url).await?,
    };

    let app = Router::new()
        .merge(get_user_routes().merge(get_auth_routes()))
        .with_state(state);

    let api_port = std::env::var("API_PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .unwrap_or(5000);
    info!("Starting server at port {}", api_port);

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], api_port));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ApiError {
    InternalServerError { error: String },
    BadRequest { error: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InternalServerError { error } => {
                error!("{}", error);
                (StatusCode::BAD_REQUEST, Json(error)).into_response()
            }
            ApiError::BadRequest { error } => {
                error!("{}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
            }
        }
    }
}

pub trait ApplicationState: Clone + Send + Sync {
    fn get_pg_pool_ref(&self) -> &PgPool;
}

#[derive(Clone, Debug)]
struct AppState {
    pool: PgPool,
}

impl ApplicationState for AppState {
    fn get_pg_pool_ref(&self) -> &PgPool {
        &self.pool
    }
}
