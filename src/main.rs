use actix_web::{web, App, HttpServer};
use rust::{users, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    let api_host = std::env::var("API_HOST").unwrap_or("127.0.0.1".to_string());
    let api_port = std::env::var("API_PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("API_PORT deve ser um número inteiro entre 0 e 65535");

    let app_state = AppState::default().await;

    match HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(users::controller())
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
