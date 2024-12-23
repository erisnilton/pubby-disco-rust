use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use sqlx::{postgres::PgPoolOptions, Postgres};

pub mod di;
pub mod domain;
pub mod infra;
pub mod shared;

#[derive(Clone)]
pub struct AppState {
  pub db: sqlx::Pool<Postgres>,
  pub in_memory: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl AppState {
  pub async fn default() -> Self {
    // Get DATABASE_URL from .env file
    let db_url = match std::env::var("DATABASE_URL") {
      Ok(url) => url,
      Err(_) => {
        eprintln!("🔥 DATABASE_URL not defined");
        std::process::exit(1);
      }
    };

    // Connect to database with max connections 100
    let pool = match PgPoolOptions::new()
      .max_connections(4)
      .min_connections(1)
      .connect(&db_url)
      .await
    {
      Ok(pool) => {
        println!("✅ Database connected successfully!");
        pool
      }
      Err(e) => {
        eprintln!("🔥 Failed to connect to database: {}", e);
        std::process::exit(1);
      }
    };

    Self {
      db: pool,
      in_memory: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}
