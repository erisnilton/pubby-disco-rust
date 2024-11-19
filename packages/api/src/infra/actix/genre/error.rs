use crate::{
  domain::{self, genre::Error},
  infra::actix::errors::ErrorResponse,
};

impl From<Error> for ErrorResponse {
  fn from(value: Error) -> Self {
    match value {
      Error::DatabaseError(err) => ErrorResponse::InternalServerError(err.to_string()),
    }
  }
}

impl From<domain::genre::stories::contribute::Error> for ErrorResponse {
  fn from(value: domain::genre::stories::contribute::Error) -> Self {
    match value {
      domain::genre::stories::contribute::Error::ActivityRepositoryError(error) => error.into(),
      domain::genre::stories::contribute::Error::GenreNotFound => {
        ErrorResponse::NotFound(String::from("Genre not found"))
      }
      domain::genre::stories::contribute::Error::GenreRepositoryError(error) => error.into(),
    }
  }
}

impl From<domain::genre::stories::apply_changes::Error> for ErrorResponse {
  fn from(value: domain::genre::stories::apply_changes::Error) -> Self {
    match value {
      domain::genre::stories::apply_changes::Error::EntityIsNotGenre => {
        ErrorResponse::BadRequest(String::from("Entity is not a genre"), None)
      }
      domain::genre::stories::apply_changes::Error::RepositoryError(error) => error.into(),
    }
  }
}
