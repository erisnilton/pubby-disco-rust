#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use actix_session::SessionMiddleware;
use actix_web::{cookie::Key, error::JsonPayloadError, web, App, Error, HttpResponse, HttpServer};
use base64::Engine;
use rust::{infra, AppState};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Load .env file
  dotenvy::dotenv().ok();

  // Initialize logger
  env_logger::init();

  let api_host = std::env::var("API_HOST").unwrap_or("127.0.0.1".to_string());
  let api_port = match std::env::var("API_PORT")
    .unwrap_or("8080".to_string())
    .parse::<u16>()
  {
    Ok(port) => port,
    Err(_) => {
      eprintln!("🔥 API_PORT deve ser um número inteiro entre 0 e 65535");

      std::process::exit(1);
    }
  };

  let app_state = AppState::default().await;
  let key = Key::from(
    base64::prelude::BASE64_STANDARD
      .decode(
        std::env::var("COOKIE_SECRET")
          .expect("COOKIE_SECRET must be set")
          .as_str(),
      )
      .expect("COOKIE_SECRET must be a valid base64 string")
      .as_slice(),
  );

  match HttpServer::new(move || {
    App::new()
      .wrap(SessionMiddleware::new(
        actix_session::storage::CookieSessionStore::default(),
        key.clone(),
      ))
      .app_data(web::Data::new(app_state.clone()))
      .app_data(
        web::JsonConfig::default()
          .limit(4096)
          .error_handler(|error, _req| match error {
            JsonPayloadError::ContentType => actix_web::error::ErrorNotAcceptable(json!({
              "name": "NotAcceptable",
              "message": "Content-Type must be application/json"
            })),
            JsonPayloadError::Deserialize(json_error) => actix_web::error::ErrorBadRequest(json!({
              "name": "BadRequest",
              "message": "Invalid JSON",
              "details": json_error.to_string()
            })),
            _ => actix_web::error::ErrorInternalServerError(json!({
              "name": "InternalServerError",
              "message": "Internal Server Error"
            })),
          }),
      )
      .service(infra::actix::user::controller())
      .service(infra::actix::activity::controller())
  })
  .bind((api_host.as_str(), api_port))
  {
    Ok(server) => {
      println!("🚀 Server running at http://{}:{}", api_host, api_port);
      server
    }
    Err(e) => {
      eprintln!("🔥 Failed to bind server: {}", e);
      std::process::exit(1);
    }
  }
  .run()
  .await?;

  println!("\n🛑 Server stopped");

  Ok(())
}
