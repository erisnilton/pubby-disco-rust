use crate::{
  domain::activity::{
    stories::reject::RejectActivityError, ActivityError, ActivityRepositoryError,
  },
  infra::actix::errors::ErrorResponse,
};

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

impl From<ActivityError> for ErrorResponse {
  fn from(value: ActivityError) -> Self {
    match value {
      ActivityError::ActivityIsNotPending => {
        ErrorResponse::BadRequest(String::from("Activity is not pending"), None)
      }
    }
  }
}

impl From<RejectActivityError> for ErrorResponse {
  fn from(value: RejectActivityError) -> Self {
    match value {
      RejectActivityError::ActivityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      RejectActivityError::RepositoryError(error) => error.into(),
      RejectActivityError::UserIsNotACurator => {
        ErrorResponse::Forbidden(String::from("User is not a curator"))
      }
      RejectActivityError::ActivityError(error) => error.into(),
    }
  }
}
