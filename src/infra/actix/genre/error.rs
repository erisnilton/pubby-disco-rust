use crate::{domain::genre::GenreRepositoryError, infra::actix::errors::ErrorResponse};

impl From<GenreRepositoryError> for ErrorResponse {
  fn from(value: GenreRepositoryError) -> Self {
    match value {
      GenreRepositoryError::DatabaseError(err) => {
        ErrorResponse::InternalServerError(err.to_string())
      }
    }
  }
}
