mod errors;
mod middlewares;
mod routes;

use crate::middlewares::validate_body;
use axum::{
    body, middleware,
    routing::{get, post},
    Router,
};
use hyper::{header, Method};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda};
use routes::print_body;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{self, CorsLayer},
    trace::TraceLayer,
    ServiceBuilderExt,
};
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
    let app = Router::new()
        .route("/name", post(print_body))
        // add this middleware to just the above routes, the order is bottom to top
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn(validate_body)),
        )
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            CorsLayer::new()
                // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin(cors::Any)
                .allow_methods([Method::GET, Method::POST])
                // It is required to add ".allow_headers([http::header::CONTENT_TYPE])"
                // See this issue https://github.com/tokio-rs/axum/issues/849
                .allow_headers([header::CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

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
