#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::artist::repository::Error),
  EntityIsNotArtist,
}

pub type Input = domain::artist::vo::Contribution;

pub async fn execute(
  artist_repository: &mut impl crate::artist::repository::ArtistRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    Input::Create(artist) => {
      artist_repository
        .create(&artist)
        .await
        .map_err(Error::RepositoryError)?;
    }
    Input::Update {
      entity: mut artist,
      changes,
    } => {
      artist.apply_changes(&changes);
      artist_repository
        .update(&artist)
        .await
        .map_err(Error::RepositoryError)?;
    }
    Input::Delete(artist) => {
      artist_repository
        .delete_by_id(artist.id())
        .await
        .map_err(Error::RepositoryError)?;
    }
  }

  Ok(())
}
