use application::user::stories::{user_login, user_register};

use crate::errors::ErrorResponse;

impl From<user_login::LoginError> for ErrorResponse {
  fn from(value: user_login::LoginError) -> Self {
    match value {
      user_login::LoginError::InvalidCredentials => {
        ErrorResponse::Unauthorized(String::from("Invalid credentials"))
      }

      user_login::LoginError::RepositoryError(error) => {
        log::error!("Failed to login: {:?}", error);

        ErrorResponse::InternalServerError(String::from("Failed to login"))
      }
    }
  }
}

impl From<user_register::CreateUserStoryError> for ErrorResponse {
  fn from(value: user_register::CreateUserStoryError) -> Self {
    match value {
      user_register::CreateUserStoryError::UserAlreadyExists => {
        ErrorResponse::Conflict(String::from("User already exists"))
      }

      user_register::CreateUserStoryError::InvalidInput(error) => ErrorResponse::BadRequest(
        String::from("Invalid input"),
        Some(
          error
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
              (
                field.to_string(),
                errors
                  .iter()
                  .filter_map(|error| error.message.clone().map(String::from))
                  .collect(),
              )
            })
            .collect(),
        ),
      ),

      user_register::CreateUserStoryError::RepositoryError(error) => {
        log::error!("Failed to create user: {:?}", error);

        ErrorResponse::InternalServerError(String::from("Failed to create user"))
      }
    }
  }
}
