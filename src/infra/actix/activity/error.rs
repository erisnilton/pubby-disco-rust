use crate::{
  domain::{
    activity::{
      error::EntityUpdateError,
      stories::{approve::ApproveActivityError, reject::RejectActivityError},
      ActivityError, ActivityRepositoryError,
    },
    genre::stories::apply_changes::ApplyChangesError,
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

impl From<ApplyChangesError> for ErrorResponse {
  fn from(value: ApplyChangesError) -> Self {
    match value {
      ApplyChangesError::EntityIsNotGenre => {
        ErrorResponse::BadRequest(String::from("Entity is not a genre"), None)
      }
      ApplyChangesError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<EntityUpdateError> for ErrorResponse {
  fn from(value: EntityUpdateError) -> Self {
    match value {
      EntityUpdateError::Genre(error) => error.into(),
      EntityUpdateError::Artist(error) => error.into(),
    }
  }
}

impl From<ApproveActivityError> for ErrorResponse {
  fn from(value: ApproveActivityError) -> Self {
    match value {
      ApproveActivityError::ActivityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      ApproveActivityError::RepositoryError(error) => error.into(),
      ApproveActivityError::UserIsNotACurator => {
        ErrorResponse::Forbidden(String::from("User is not a curator"))
      }
      ApproveActivityError::ActivityError(error) => error.into(),
      ApproveActivityError::InvalidEntity => {
        ErrorResponse::BadRequest(String::from("Invalid entity"), None)
      }
      ApproveActivityError::EntityUpdateError(error) => error.into(),
    }
  }
}
