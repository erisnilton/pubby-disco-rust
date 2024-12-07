use application::source::{
  repository::Error as SourceRepositoryError,
  stories::{apply_changes::Error as ApplyChangesError, contribute::Error as ContributeError},
};

use crate::errors::ErrorResponse;

impl From<ApplyChangesError> for ErrorResponse {
  fn from(value: ApplyChangesError) -> Self {
    match value {
      ApplyChangesError::RepositoryError(error) => error.into(),
    }
  }
}

impl From<SourceRepositoryError> for ErrorResponse {
  fn from(value: SourceRepositoryError) -> Self {
    match value {
      SourceRepositoryError::DatabaseError(error) => {
        ErrorResponse::InternalServerError(error.to_string())
      }
    }
  }
}

impl From<ContributeError> for ErrorResponse {
  fn from(value: ContributeError) -> Self {
    match value {
      ContributeError::ActivityRepositoryError(error) => error.into(),
      ContributeError::SourceNotFound => ErrorResponse::NotFound(String::from("Source not found")),
      ContributeError::SourceRepositoryError(error) => error.into(),
    }
  }
}
