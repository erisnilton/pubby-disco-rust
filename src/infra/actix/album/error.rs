use crate::{
  domain::album::{stories::apply_changes::ApplyChangesError, AlbumRepositoryError},
  infra::actix::errors::ErrorResponse,
};

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
