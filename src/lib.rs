use sqlx::{postgres::PgPoolOptions, Postgres};

pub mod users;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::Pool<Postgres>,
}

impl AppState {
    pub async fn default() -> Self {
        // Get DATABASE_URL from .env file
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL deve ser definido nas variáveis de ambiente");

        // Connect to database with max connections 100
        let pool = match PgPoolOptions::new()
            .max_connections(100)
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

        Self { db: pool }
    }
}
