use std::{
  fmt::Debug,
  net::{IpAddr, Ipv4Addr, SocketAddr},
};

use base64::Engine;

#[derive(Debug, Clone, Copy)]
pub struct ApiConfig {
  pub address: std::net::SocketAddr,
}

impl ApiConfig {
  pub fn from_env() -> Self {
    Self {
      address: std::env::var("API_ADDR").map_or(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
        |addr| addr.parse().expect("🔥 Invalid API_ADDR"),
      ),
    }
  }
}

#[derive(Debug, Clone)]
pub struct SessionCookie {
  pub name: String,
  pub domain: Option<String>,
  pub http_only: bool,
  pub secure: bool,
}

#[derive(Clone)]
pub struct SessionConfig {
  pub ttl: i64,
  pub cookie: SessionCookie,
  pub secret: Vec<u8>,
}

impl Debug for SessionConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SessionConfig")
      .field("ttl", &self.ttl)
      .field("cookie", &self.cookie)
      .field(
        "secret",
        &base64::prelude::BASE64_STANDARD.encode(&self.secret),
      )
      .finish()
  }
}

impl SessionConfig {
  pub fn from_env() -> Self {
    let secret = base64::prelude::BASE64_STANDARD
      .decode(
        std::env::var("SESSION_SECRET")
          .expect("🔥 SESSION_SECRET not defined")
          .as_str(),
      )
      .expect("🔥 SESSION_SECRET must be a valid base64 string");

    let ttl: i64 = std::env::var("SESSION_TTL").map_or(3600, |v| {
      v.parse().expect("🔥 SESSION_TTL must be a number")
    });

    let cookie_http_only = std::env::var("SESSION_COOKIE_HTTP_ONLY").map_or(false, |v| {
      v.parse()
        .expect("🔥 SESSION_COOKIE_HTTP_ONLY must be a boolean")
    });

    let cookie_secure = std::env::var("SESSION_COOKIE_SECURE").map_or(false, |v| {
      v.parse()
        .expect("🔥 SESSION_COOKIE_SECURE must be a boolean")
    });

    let cookie = SessionCookie {
      name: std::env::var("SESSION_COOKIE_NAME").unwrap_or(String::from("session_id")),
      domain: std::env::var("SESSION_COOKIE_DOMAIN").ok(),
      http_only: cookie_http_only,
      secure: cookie_secure,
    };

    Self {
      secret,
      ttl,
      cookie,
    }
  }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
  pub api: ApiConfig,
  pub session: SessionConfig,
  pub database_url: String,
}

impl AppConfig {
  pub fn from_env() -> Self {
    dotenvy::dotenv().ok();

    let database_url = match std::env::var("DATABASE_URL") {
      Ok(url) => url,
      Err(_) => {
        eprintln!("🔥 DATABASE_URL not defined");
        std::process::exit(1);
      }
    };

    let api = ApiConfig::from_env();
    let session = SessionConfig::from_env();

    Self {
      api,
      session,
      database_url,
    }
  }
}
