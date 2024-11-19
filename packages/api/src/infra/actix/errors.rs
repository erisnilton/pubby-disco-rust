use std::{collections::HashMap, fmt::Display};

use actix_web::HttpResponse;
use serde_json::json;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]

pub enum ErrorResponse {
  Unauthorized(String),
  Conflict(String),
  BadRequest(String, Option<HashMap<String, Vec<String>>>),
  InternalServerError(String),
  NotFound(String),
  Forbidden(String),
}

impl Display for ErrorResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ErrorResponse::Unauthorized(message) => write!(f, "Unauthorized: {}", message),
      ErrorResponse::Conflict(message) => write!(f, "Conflict: {}", message),
      ErrorResponse::BadRequest(message, Some(errors)) => {
        writeln!(f, "Bad Request: {}", message)?;

        for (field, errors) in errors {
          writeln!(f, "  {}: {}", field, errors.join(", "))?;
        }

        Ok(())
      }
      ErrorResponse::BadRequest(message, None) => write!(f, "Bad Request: {}", message),
      ErrorResponse::InternalServerError(message) => {
        write!(f, "Internal Server Error: {}", message)
      }
      ErrorResponse::NotFound(message) => write!(f, "Not Found: {}", message),
      ErrorResponse::Forbidden(message) => write!(f, "Forbidden: {}", message),
    }
  }
}

impl actix_web::ResponseError for ErrorResponse {
  fn status_code(&self) -> actix_web::http::StatusCode {
    match self {
      ErrorResponse::BadRequest(_, _) => actix_web::http::StatusCode::BAD_REQUEST,
      ErrorResponse::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
      ErrorResponse::Conflict(_) => actix_web::http::StatusCode::CONFLICT,
      ErrorResponse::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
      ErrorResponse::InternalServerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
      ErrorResponse::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
    }
  }
}

impl From<ErrorResponse> for HttpResponse {
  fn from(value: ErrorResponse) -> Self {
    match value {
      ErrorResponse::BadRequest(message, details) => HttpResponse::BadRequest().json(json!({
        "name": "BadRequest",
        "message": message,
        "details": details
      })),
      ErrorResponse::Conflict(message) => HttpResponse::Conflict().json(json!({
        "name": "Conflict",
        "message": message
      })),
      ErrorResponse::NotFound(message) => HttpResponse::NotFound().json(json!({
        "name": "NotFound",
        "message": message
      })),
      ErrorResponse::Unauthorized(message) => HttpResponse::Unauthorized().json(json!({
        "name": "Unauthorized",
        "message": message
      })),
      ErrorResponse::InternalServerError(message) => {
        HttpResponse::InternalServerError().json(json!({
          "name": "InternalServerError",
          "message": message
        }))
      }
      ErrorResponse::Forbidden(message) => HttpResponse::Forbidden().json(json!({
        "name": "Forbidden",
        "message": message
      })),
    }
  }
}
