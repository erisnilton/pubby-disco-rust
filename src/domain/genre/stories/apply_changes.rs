#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::domain::genre::repository::Error),
  EntityIsNotGenre,
}

pub type Input = crate::domain::genre::contribution::Contribution;

pub async fn execute(
  repository_genre: &mut impl crate::domain::genre::GenreRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    crate::domain::genre::contribution::Contribution::Create(genre) => {
      repository_genre
        .create(genre)
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
    crate::domain::genre::contribution::Contribution::Update {
      entity: mut genre,
      changes,
    } => {
      genre.apply_changes(&changes);
      repository_genre
        .update(genre)
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
    crate::domain::genre::contribution::Contribution::Delete(genre) => {
      repository_genre
        .delete_by_id(&genre.id())
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
  }
}
