use crate::errors::ErrorResponse;

use application::genre::{
  repository::Error as GenreRepositoryError,
  stories::{
    apply_changes::Error as ApplyChangesError, contribute::Error as ContributeError,
    find_all::Error as FindAllError, find_by_slug::Error as FindBySlugError,
  },
};

impl From<GenreRepositoryError> for ErrorResponse {
  fn from(value: GenreRepositoryError) -> Self {
    match value {
      GenreRepositoryError::DatabaseError(err) => {
        ErrorResponse::InternalServerError(err.to_string())
      }
    }
  }
}

impl From<ContributeError> for ErrorResponse {
  fn from(value: ContributeError) -> Self {
    match value {
      ContributeError::ActivityRepositoryError(error) => error.into(),
      ContributeError::GenreNotFound => ErrorResponse::NotFound(String::from("Genre not found")),
      ContributeError::GenreRepositoryError(error) => error.into(),
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

impl From<FindBySlugError> for ErrorResponse {
  fn from(value: FindBySlugError) -> Self {
    match value {
      FindBySlugError::GenreNotFound => ErrorResponse::NotFound(String::from("Genre not found")),
      FindBySlugError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<FindAllError> for ErrorResponse {
  fn from(value: FindAllError) -> Self {
    match value {
      FindAllError::RepositoryError(err) => err.into(),
    }
  }
}
