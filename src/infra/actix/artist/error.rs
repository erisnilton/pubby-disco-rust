use crate::{
  domain::artists::{repository::ArtistRepositoryError, stories::apply_changes::ApplyChangesError},
  infra::actix::errors::ErrorResponse,
};

impl From<ArtistRepositoryError> for ErrorResponse {
  fn from(value: ArtistRepositoryError) -> Self {
    match value {
      ArtistRepositoryError::Conflict(err) => ErrorResponse::Conflict(err.to_string()),
      ArtistRepositoryError::DatabaseError(err) => {
        ErrorResponse::InternalServerError(err.to_string())
      }
      ArtistRepositoryError::NotFound => ErrorResponse::NotFound("Artist not found".to_string()),
    }
  }
}

impl From<ApplyChangesError> for ErrorResponse {
  fn from(value: ApplyChangesError) -> Self {
    match value {
      ApplyChangesError::EntityIsNotArtist => {
        ErrorResponse::BadRequest("Entity is not an artist".to_string(), None)
      }
      ApplyChangesError::RepositoryError(error) => error.into(),
    }
  }
}
