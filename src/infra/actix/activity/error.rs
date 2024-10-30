use crate::{domain::activity::ActivityRepositoryError, infra::actix::errors::ErrorResponse};

impl From<ActivityRepositoryError> for ErrorResponse {
  fn from(value: ActivityRepositoryError) -> Self {
    match value {
      ActivityRepositoryError::EntityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      ActivityRepositoryError::InternalServerError(message) => {
        ErrorResponse::InternalServerError(message)
      }
    }
  }
}
