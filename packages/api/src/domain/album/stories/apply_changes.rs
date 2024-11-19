#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::domain::album::repository::Error),
  EntityIsNotAlbum,
}

pub type Input = crate::domain::album::contribution::Contribution;

pub async fn execute(
  album_repository: &mut impl crate::domain::album::repository::AlbumRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    crate::domain::album::contribution::Contribution::Create(album) => {
      album_repository
        .create(&album)
        .await
        .map_err(Error::RepositoryError)?;
    }
    crate::domain::album::contribution::Contribution::Update {
      entity: mut album,
      changes,
    } => {
      album.apply_changes(&changes);
      album_repository
        .update(&album)
        .await
        .map_err(Error::RepositoryError)?;
    }
    crate::domain::album::contribution::Contribution::Delete(album) => {
      album_repository
        .delete_by_id(&album.id)
        .await
        .map_err(Error::RepositoryError)?;
    }
  }

  Ok(())
}
