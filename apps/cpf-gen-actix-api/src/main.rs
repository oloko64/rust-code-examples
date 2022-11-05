use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
mod paths;
mod types;
mod utils;

const PORT: u16 = 8080;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:{}", PORT);
    println!("API Version: {}", env!("CARGO_PKG_VERSION"));
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allowed_methods(vec!["GET"]).allowed_header(header::CONTENT_TYPE)
        .max_age(3600);
        App::new()
        .wrap(cors)
            .route("/", web::get().to(|| async { "CPF Generator and Validator API. For more information see https://github.com/OLoKo64/actix-cpf-generator-api" }))
            .service(paths::new_cpf)
            .service(paths::validate_cpf)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
