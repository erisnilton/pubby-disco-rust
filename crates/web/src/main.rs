#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod activity;
mod album;
mod app_state;
mod artist;
mod di;
mod errors;
mod genre;
mod media;
mod page;
mod source;
mod user;
mod utils;

use actix_files::Files;
use actix_session::SessionMiddleware;
use actix_web::{
  cookie::Key,
  error::JsonPayloadError,
  middleware::NormalizePath,
  web::{self},
  App, HttpServer,
};

use app_state::AppState;
use serde_json::json;

use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config = config::AppConfig::from_env();

  // Initialize logger
  env_logger::init();

  let app_state = AppState::default().await;
  let address = config.api.address;
  let key = Key::from(config.session.secret.as_slice());

  match HttpServer::new(move || {
    let config = config.clone();
    App::new()
      .wrap(NormalizePath::trim())
      .wrap(Logger::default())
      .wrap(
        SessionMiddleware::builder(
          actix_session::storage::CookieSessionStore::default(),
          key.clone(),
        )
        .cookie_name(config.session.cookie.name)
        .cookie_domain(config.session.cookie.domain)
        .cookie_http_only(config.session.cookie.http_only)
        .cookie_secure(config.session.cookie.secure)
        .session_lifecycle(
          actix_session::config::PersistentSession::default()
            .session_ttl(actix_web::cookie::time::Duration::seconds(
              config.session.ttl,
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
              "message": "Invalid Request JSON",
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
        crate::artist::controller::configure(config);
        crate::user::controller::configure(config);
        crate::activity::controller::configure(config);
        crate::genre::controller::configure(config);
        crate::album::controller::configure(config);
        crate::source::controller::configure(config);
        crate::media::controller::configure(config);
      })
  })
  .bind(address)
  {
    Ok(server) => {
      println!("🚀 Server running at http://{}", address);
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
