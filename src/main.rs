#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use actix_web::{web, App, HttpServer};
use rust::{ infra, users, AppState};

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

  match HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(app_state.clone()))
      .service(users::controller())
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
