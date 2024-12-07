use application::album::{
  repository::Error as AlbumRepositoryError,
  stories::{
    apply_changes::Error as ApplyChangesError, contribute::Error as ContributeError,
    find_by::Error as FindByError, find_by_slug::Error as FindBySlugError,
  },
};

use crate::errors::ErrorResponse;
impl From<ApplyChangesError> for ErrorResponse {
  fn from(value: ApplyChangesError) -> Self {
    match value {
      ApplyChangesError::EntityIsNotAlbum => {
        ErrorResponse::BadRequest(String::from("Entity is not an album"), None)
      }
      ApplyChangesError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<AlbumRepositoryError> for ErrorResponse {
  fn from(value: AlbumRepositoryError) -> Self {
    match value {
      AlbumRepositoryError::DatabaseError(error) => {
        ErrorResponse::InternalServerError(error.to_string())
      }
    }
  }
}

impl From<ContributeError> for ErrorResponse {
  fn from(value: ContributeError) -> Self {
    match value {
      ContributeError::ActivityRepositoryError(error) => error.into(),
      ContributeError::AlbumNotFound => ErrorResponse::NotFound(String::from("Album not found")),
      ContributeError::AlbumRepositoryError(error) => error.into(),
    }
  }
}

impl From<FindBySlugError> for ErrorResponse {
  fn from(value: FindBySlugError) -> Self {
    match value {
      FindBySlugError::AlbumNotFound => ErrorResponse::NotFound(String::from("Album not found")),
      FindBySlugError::RepositoryError(error) => error.into(),
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
