use infra::actix::errors::ErrorResponse;

use crate::*;

impl From<domain::artist::stories::contribute::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::artist::stories::contribute::Error) -> Self {
    match value {
      domain::artist::stories::contribute::Error::ActivityRepositoryError(error) => error.into(),
      domain::artist::stories::contribute::Error::ArtistNotFound => {
        infra::actix::errors::ErrorResponse::NotFound(String::from("Artist not found"))
      }
      domain::artist::stories::contribute::Error::ArtistRepositoryError(error) => error.into(),
    }
  }
}

impl From<domain::artist::stories::apply_changes::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::artist::stories::apply_changes::Error) -> Self {
    match value {
      domain::artist::stories::apply_changes::Error::EntityIsNotArtist => {
        infra::actix::errors::ErrorResponse::BadRequest(
          String::from("Entity is not an artist"),
          None,
        )
      }
      domain::artist::stories::apply_changes::Error::RepositoryError(error) => error.into(),
    }
  }
}

impl From<domain::artist::repository::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::artist::repository::Error) -> Self {
    match value {
      domain::artist::repository::Error::DatabaseError(error) => {
        infra::actix::errors::ErrorResponse::InternalServerError(error.to_string())
      }
      domain::artist::repository::Error::Conflict(error) => {
        infra::actix::errors::ErrorResponse::Conflict(error.to_string())
      }
    }
  }
}



impl From<domain::artist::stories::find_by_slug::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::artist::stories::find_by_slug::Error) -> Self {
    match value {
      domain::artist::stories::find_by_slug::Error::ArtistNotFound => {
        ErrorResponse::NotFound(String::from("Artist not found"))
      }
      domain::artist::stories::find_by_slug::Error::RepositoryError(error) => error.into(),
    }
  }
}
