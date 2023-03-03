mod errors;
mod middlewares;
mod routes;

use lambda_web::{is_running_on_lambda, run_hyper_on_lambda};
use std::net::SocketAddr;
use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

const PORT: u16 = 3000;
const ADDR_IPV4: [u8; 4] = [0, 0, 0, 0];

#[tokio::main]
async fn main() {
    // load .env file
    dotenvy::dotenv().ok();

    // initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = routes::get_app();

    if is_running_on_lambda() {
        info!("Starting server on AWS lambda");
        run_hyper_on_lambda(app).await.unwrap();
    } else {
        info!("Starting server on http://localhost:{}", PORT);
        // run it with hyper on localhost:3000
        let addr = SocketAddr::from((ADDR_IPV4, PORT));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
