#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::domain::artist::repository::Error),
  EntityIsNotArtist,
}

pub type Input = crate::domain::artist::contribution::Contribution;

pub async fn execute(
  artist_repository: &mut impl crate::domain::artist::repository::ArtistRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    crate::domain::artist::contribution::Contribution::Create(artist) => {
      artist_repository
        .create(&artist)
        .await
        .map_err(Error::RepositoryError)?;
    }
    crate::domain::artist::contribution::Contribution::Update {
      entity: mut artist,
      changes,
    } => {
      artist.apply_changes(&changes);
      artist_repository
        .update(&artist)
        .await
        .map_err(Error::RepositoryError)?;
    }
    crate::domain::artist::contribution::Contribution::Delete(artist) => {
      artist_repository
        .delete_by_id(artist.id())
        .await
        .map_err(Error::RepositoryError)?;
    }
  }

  Ok(())
}
