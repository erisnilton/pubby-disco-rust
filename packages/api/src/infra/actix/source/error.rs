impl From<crate::domain::source::stories::apply_changes::Error>
  for crate::infra::actix::errors::ErrorResponse
{
  fn from(value: crate::domain::source::stories::apply_changes::Error) -> Self {
    match value {
      crate::domain::source::stories::apply_changes::Error::RepositoryError(error) => error.into(),
    }
  }
}

impl From<crate::domain::source::repository::Error> for crate::infra::actix::errors::ErrorResponse {
  fn from(value: crate::domain::source::repository::Error) -> Self {
    match value {
      crate::domain::source::repository::Error::DatabaseError(error) => {
        crate::infra::actix::errors::ErrorResponse::InternalServerError(error.to_string())
      }
    }
  }
}

impl From<crate::domain::source::stories::contribute::Error>
  for crate::infra::actix::errors::ErrorResponse
{
  fn from(value: crate::domain::source::stories::contribute::Error) -> Self {
    match value {
      crate::domain::source::stories::contribute::Error::ActivityRepositoryError(error) => {
        error.into()
      }
      crate::domain::source::stories::contribute::Error::SourceNotFound => {
        crate::infra::actix::errors::ErrorResponse::NotFound(String::from("Source not found"))
      }
      crate::domain::source::stories::contribute::Error::SourceRepositoryError(error) => {
        error.into()
      }
    }
  }
}
