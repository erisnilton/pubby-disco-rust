use crate::errors::ErrorResponse;

use application::artist::{
  repository::Error as ArtistRepositoryError,
  stories::{
    apply_changes::Error as ApplyChangesError, contribute::Error as ContributeError,
    find_all::Error as FindAllError, find_by_slug::Error as FindBySlugError,
  },
};

impl From<ContributeError> for ErrorResponse {
  fn from(value: ContributeError) -> Self {
    match value {
      ContributeError::ActivityRepositoryError(error) => error.into(),
      ContributeError::ArtistNotFound => ErrorResponse::NotFound(String::from("Artist not found")),
      ContributeError::ArtistRepositoryError(error) => error.into(),
    }
  }
}

impl From<ApplyChangesError> for ErrorResponse {
  fn from(value: ApplyChangesError) -> Self {
    match value {
      ApplyChangesError::EntityIsNotArtist => {
        ErrorResponse::BadRequest(String::from("Entity is not an artist"), None)
      }
      ApplyChangesError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<ArtistRepositoryError> for ErrorResponse {
  fn from(value: ArtistRepositoryError) -> Self {
    match value {
      ArtistRepositoryError::DatabaseError(error) => {
        ErrorResponse::InternalServerError(error.to_string())
      }
      ArtistRepositoryError::Conflict(error) => ErrorResponse::Conflict(error.to_string()),
    }
  }
}

impl From<FindAllError> for ErrorResponse {
  fn from(value: FindAllError) -> Self {
    match value {
      FindAllError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<FindBySlugError> for ErrorResponse {
  fn from(value: FindBySlugError) -> Self {
    match value {
      FindBySlugError::ArtistNotFound => ErrorResponse::NotFound(String::from("Artist not found")),
      FindBySlugError::RepositoryError(error) => error.into(),
    }
  }
}
