use crate::errors::ErrorResponse;
use application::activity::{
  repository::Error as ActivityRepositoryError,
  stories::{approve::Error as ApproveError, reject::Error as RejectError},
};
use domain::activity::entity::Error as ActivityError;

impl From<ActivityRepositoryError> for ErrorResponse {
  fn from(value: ActivityRepositoryError) -> Self {
    match value {
      ActivityRepositoryError::EntityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      ActivityRepositoryError::InternalServerError(err) => {
        ErrorResponse::InternalServerError(err.to_string())
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

impl From<ApproveError> for ErrorResponse {
  fn from(value: ApproveError) -> Self {
    match value {
      ApproveError::ActivityError(error) => error.into(),
      ApproveError::ActivityNotFound => ErrorResponse::NotFound(String::from("Activity not found")),
      ApproveError::AlbumApplyError(error) => error.into(),
      ApproveError::ArtistApplyError(error) => error.into(),
      ApproveError::GenreApplyError(error) => error.into(),
      ApproveError::RepositoryError(error) => error.into(),
      ApproveError::UserIsNotACurator => {
        ErrorResponse::Forbidden(String::from("User is not a curator"))
      }
      ApproveError::MediaApplyError(error) => error.into(),
      ApproveError::SourceApplyError(error) => error.into(),
    }
  }
}

impl From<RejectError> for ErrorResponse {
  fn from(value: RejectError) -> Self {
    match value {
      RejectError::ActivityError(error) => error.into(),
      RejectError::ActivityNotFound => ErrorResponse::NotFound(String::from("Activity not found")),
      RejectError::ActivityRepositoryError(error) => error.into(),
      RejectError::UserIsNotACurator => {
        ErrorResponse::Forbidden(String::from("User is not a curator"))
      }
    }
  }
}
