use application::media::{
  repository::Error as MediaRepositoryError,
  stories::{
    apply_changes::Error as ApplyChangesError, contribute::Error as ContributeError,
    find_by::Error as FindByError,
  },
};

use crate::errors::ErrorResponse;

impl From<ApplyChangesError> for ErrorResponse {
  fn from(value: ApplyChangesError) -> Self {
    match value {
      ApplyChangesError::AlbumRepositoryError(error) => error.into(),
      ApplyChangesError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<MediaRepositoryError> for ErrorResponse {
  fn from(value: MediaRepositoryError) -> Self {
    match value {
      MediaRepositoryError::DatabaseError(error) => ErrorResponse::InternalServerError(error),
    }
  }
}

impl From<ContributeError> for ErrorResponse {
  fn from(value: ContributeError) -> Self {
    match value {
      ContributeError::ActivityRepositoryError(error) => error.into(),
      ContributeError::MediaNotFound => ErrorResponse::NotFound(String::from("Media not found")),
      ContributeError::MediaRepositoryError(error) => error.into(),
    }
  }
}

impl From<FindByError> for ErrorResponse {
  fn from(value: FindByError) -> Self {
    match value {
      FindByError::RepositoryError(error) => error.into(),
    }
  }
}
