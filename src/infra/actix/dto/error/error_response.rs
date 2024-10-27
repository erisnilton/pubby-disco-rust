#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
  pub name: String,
  pub message: String,
  pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
  pub fn unauthorized(message: impl Into<String>) -> Self {
    ErrorResponse {
      name: String::from("Unauthorized"),
      message: message.into(),
      details: None,
    }
  }

  pub fn internal_server_error(message: impl Into<String>) -> Self {
    ErrorResponse {
      name: String::from("InternalServerError"),
      message: message.into(),
      details: None,
    }
  }

  pub fn conflict(message: impl Into<String>) -> Self {
    ErrorResponse {
      name: String::from("Conflict"),
      message: message.into(),
      details: None,
    }
  }

  pub fn bad_request(message: impl Into<String>, details: serde_json::Value) -> Self {
    ErrorResponse {
      name: String::from("BadRequest"),
      message: message.into(),
      details: Some(details),
    }
  }
}
