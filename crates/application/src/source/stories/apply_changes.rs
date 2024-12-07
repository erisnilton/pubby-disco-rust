use crate::source::repository::SourceRepository;

#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::source::repository::Error),
}

pub type Input = domain::source::vo::Contribution;

pub async fn execute(
  source_repository: &mut impl SourceRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    Input::Create(source) => {
      source_repository
        .create(&source)
        .await
        .map_err(Error::RepositoryError)?;

      Ok(())
    }
    Input::Delete(source) => {
      source_repository
        .delete_by_id(source.id())
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
  }
}
