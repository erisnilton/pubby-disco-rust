use domain::album::album_aggregate::AlbumAggregate;

use crate::*;

use super::presenter::AlbumAggregatePresenter;

impl From<domain::album::stories::apply_changes::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::album::stories::apply_changes::Error) -> Self {
    match value {
      domain::album::stories::apply_changes::Error::EntityIsNotAlbum => {
        infra::actix::errors::ErrorResponse::BadRequest(
          String::from("Entity is not an album"),
          None,
        )
      }
      domain::album::stories::apply_changes::Error::RepositoryError(error) => error.into(),
    }
  }
}

impl From<domain::album::repository::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::album::repository::Error) -> Self {
    match value {
      domain::album::repository::Error::DatabaseError(error) => {
        infra::actix::errors::ErrorResponse::InternalServerError(error.to_string())
      }
    }
  }
}

impl From<domain::album::stories::contribute::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::album::stories::contribute::Error) -> Self {
    match value {
      domain::album::stories::contribute::Error::ActivityRepositoryError(error) => error.into(),
      domain::album::stories::contribute::Error::AlbumNotFound => {
        infra::actix::errors::ErrorResponse::NotFound(String::from("Album not found"))
      }
      domain::album::stories::contribute::Error::AlbumRepositoryError(error) => error.into(),
    }
  }
}

impl From<domain::album::stories::find_by_slug::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::album::stories::find_by_slug::Error) -> Self {
    match value {
      domain::album::stories::find_by_slug::Error::AlbumNotFound => {
        infra::actix::errors::ErrorResponse::NotFound(String::from("Album not found"))
      }
      domain::album::stories::find_by_slug::Error::RepositoryError(error) => error.into(),
    }
  }
}

impl From<domain::album::stories::find_by::Error> for infra::actix::errors::ErrorResponse {
  fn from(value: domain::album::stories::find_by::Error) -> Self {
    match value {
      domain::album::stories::find_by::Error::RepositoryError(error) => error.into(),
    }
  }
}
