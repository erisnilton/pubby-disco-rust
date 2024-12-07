use crate::genre::repository::GenreRepository;

#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::genre::repository::Error),
  EntityIsNotGenre,
}

pub type Input = domain::genre::vo::Contribution;

pub async fn execute(
  repository_genre: &mut impl GenreRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    domain::genre::vo::Contribution::Create(genre) => {
      repository_genre
        .create(&genre)
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
    domain::genre::vo::Contribution::Update {
      entity: mut genre,
      changes,
    } => {
      genre.apply_changes(&changes);
      repository_genre
        .update(&genre)
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
    domain::genre::vo::Contribution::Delete(genre) => {
      repository_genre
        .delete_by_id(genre.id())
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
  }
}
