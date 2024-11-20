use crate::{domain, infra::actix::errors::ErrorResponse};

impl From<domain::activity::repository::Error> for ErrorResponse {
  fn from(value: domain::activity::repository::Error) -> Self {
    match value {
      domain::activity::repository::Error::EntityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      domain::activity::repository::Error::InternalServerError(err) => {
        ErrorResponse::InternalServerError(err.to_string())
      }
    }
  }
}

impl From<domain::activity::Error> for ErrorResponse {
  fn from(value: domain::activity::Error) -> Self {
    match value {
      domain::activity::Error::ActivityIsNotPending => {
        ErrorResponse::BadRequest(String::from("Activity is not pending"), None)
      }
    }
  }
}

impl From<domain::activity::stories::approve::Error> for ErrorResponse {
  fn from(value: domain::activity::stories::approve::Error) -> Self {
    match value {
      domain::activity::stories::approve::Error::ActivityError(error) => error.into(),
      domain::activity::stories::approve::Error::ActivityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      domain::activity::stories::approve::Error::AlbumApplyError(error) => error.into(),
      domain::activity::stories::approve::Error::ArtistApplyError(error) => error.into(),
      domain::activity::stories::approve::Error::GenreApplyError(error) => error.into(),
      domain::activity::stories::approve::Error::RepositoryError(error) => error.into(),
      domain::activity::stories::approve::Error::UserIsNotACurator => {
        ErrorResponse::Forbidden(String::from("User is not a curator"))
      }
      domain::activity::stories::approve::Error::MediaApplyError(error) => error.into(),
    }
  }
}

impl From<domain::activity::stories::reject::Error> for ErrorResponse {
  fn from(value: domain::activity::stories::reject::Error) -> Self {
    match value {
      domain::activity::stories::reject::Error::ActivityError(error) => error.into(),
      domain::activity::stories::reject::Error::ActivityNotFound => {
        ErrorResponse::NotFound(String::from("Activity not found"))
      }
      domain::activity::stories::reject::Error::ActivityRepositoryError(error) => error.into(),
      domain::activity::stories::reject::Error::UserIsNotACurator => {
        ErrorResponse::Forbidden(String::from("User is not a curator"))
      }
    }
  }
}
