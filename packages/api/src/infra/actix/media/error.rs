impl From<crate::domain::media::stories::apply_changes::Error>
  for crate::infra::actix::errors::ErrorResponse
{
  fn from(value: crate::domain::media::stories::apply_changes::Error) -> Self {
    match value {
      crate::domain::media::stories::apply_changes::Error::AlbumRepositoryError(error) => {
        error.into()
      }
      crate::domain::media::stories::apply_changes::Error::RepositoryError(error) => error.into(),
    }
  }
}

impl From<crate::domain::media::repository::Error> for crate::infra::actix::errors::ErrorResponse {
  fn from(value: crate::domain::media::repository::Error) -> Self {
    match value {
      crate::domain::media::repository::Error::DatabaseError(error) => {
        crate::infra::actix::errors::ErrorResponse::InternalServerError(error)
      }
    }
  }
}

impl From<crate::domain::media::stories::contribute::Error>
  for crate::infra::actix::errors::ErrorResponse
{
  fn from(value: crate::domain::media::stories::contribute::Error) -> Self {
    match value {
      crate::domain::media::stories::contribute::Error::ActivityRepositoryError(error) => {
        error.into()
      }
      crate::domain::media::stories::contribute::Error::MediaNotFound => {
        crate::infra::actix::errors::ErrorResponse::NotFound(String::from("Media not found"))
      }
      crate::domain::media::stories::contribute::Error::MediaRepositoryError(error) => error.into(),
    }
  }
}
