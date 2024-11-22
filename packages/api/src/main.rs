#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use actix_files::Files;
use actix_session::SessionMiddleware;
use actix_web::{
  cookie::Key,
  error::JsonPayloadError,
  middleware::NormalizePath,
  web::{self},
  App, HttpServer,
};

use api::{infra, AppState};
use base64::Engine;
use serde_json::json;

use actix_web::middleware::Logger;

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
        std::env::var("SESSION_SECRET")
          .expect("SESSION_SECRET must be set")
          .as_str(),
      )
      .expect("SESSION_SECRET must be a valid base64 string")
      .as_slice(),
  );

  match HttpServer::new(move || {
    App::new()
      .wrap(NormalizePath::trim())
      .wrap(Logger::default())
      .wrap(
        SessionMiddleware::builder(
          actix_session::storage::CookieSessionStore::default(),
          key.clone(),
        )
        .cookie_name(std::env::var("SESSION_COOKIE_NAME").unwrap_or("session_id".to_string()))
        .cookie_domain(std::env::var("SESSION_COOKIE_DOMAIN").ok())
        .cookie_http_only(
          std::env::var("SESSION_COOKIE_HTTP_ONLY")
            .unwrap_or("true".to_string())
            .parse::<bool>()
            .unwrap(),
        )
        .cookie_secure(
          std::env::var("SESSION_COOKIE_SECURE")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap(),
        )
        .session_lifecycle(
          actix_session::config::PersistentSession::default()
            .session_ttl(actix_web::cookie::time::Duration::seconds(
              std::env::var("SESSION_TTL")
                .unwrap_or("86400".to_string())
                .parse()
                .unwrap(),
            ))
            .session_ttl_extension_policy(
              actix_session::config::TtlExtensionPolicy::OnEveryRequest,
            ),
        )
        .build(),
      )
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
      .service(
        Files::new("/docs", "./swagger-ui")
          .prefer_utf8(true)
          .index_file("index.html"),
      )
      .configure(|config| {
        infra::actix::user::controller::configure(config);
        infra::actix::activity::controller::configure(config);
        infra::actix::genre::controller::configure(config);
        infra::actix::artist::controller::configure(config);
        infra::actix::album::controller::configure(config);
        infra::actix::media::controller::configure(config);
        infra::actix::source::controller::configure(config);
      })
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
