use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rust::users;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    let api_host = std::env::var("API_HOST").unwrap_or("127.0.0.1".to_string());
    let api_port = std::env::var("API_PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("API_PORT deve ser um número inteiro entre 0 e 65535");

    // Get DATABASE_URL from .env file
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL deve ser definido nas variáveis de ambiente");

    // Connect to database with max connections 100
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(db_url.as_str());

    HttpServer::new(move || App::new().service(users::controller()))
        .bind((api_host.as_str(), api_port))?
        .run()
        .await
}
