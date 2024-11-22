#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::domain::source::repository::Error),
}

pub type Input = crate::domain::source::contribution::Contribution;

pub async fn execute(
  source_repository: &mut impl crate::domain::source::repository::SourceRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    crate::domain::source::contribution::Contribution::Create(source) => {
      source_repository
        .create(&source)
        .await
        .map_err(Error::RepositoryError)?;

      Ok(())
    }
    crate::domain::source::contribution::Contribution::Delete(source) => {
      source_repository
        .delete_by_id(&source.id())
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
  }
}
