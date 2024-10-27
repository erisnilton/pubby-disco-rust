use actix_web::HttpResponse;

use crate::domain::user::stories::{user_login, user_register};

use super::ErrorResponse;

impl From<user_login::LoginError> for HttpResponse {
  fn from(value: user_login::LoginError) -> Self {
    match value {
      user_login::LoginError::InvalidCredentials => {
        HttpResponse::Unauthorized().json(ErrorResponse::unauthorized("Invalid credentials"))
      }

      user_login::LoginError::RepositoryError(error) => {
        log::error!("Failed to login: {:?}", error);

        HttpResponse::InternalServerError()
          .json(ErrorResponse::internal_server_error("Failed to login"))
      }
    }
  }
}

impl From<user_register::CreateUserStoryError> for HttpResponse {
  fn from(value: user_register::CreateUserStoryError) -> Self {
    match value {
      user_register::CreateUserStoryError::UserAlreadyExists => {
        HttpResponse::Conflict().json(ErrorResponse::conflict("User already exists"))
      }

      user_register::CreateUserStoryError::InvalidInput(error) => HttpResponse::BadRequest().json(
        ErrorResponse::bad_request("Invalid input", error.to_string().into()),
      ),

      user_register::CreateUserStoryError::RepositoryError(error) => {
        log::error!("Failed to create user: {:?}", error);

        HttpResponse::InternalServerError().json(ErrorResponse::internal_server_error(
          "Failed to create user",
        ))
      }
    }
  }
}
