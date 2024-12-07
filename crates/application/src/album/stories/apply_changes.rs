#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::album::repository::Error),
  EntityIsNotAlbum,
}

pub type Input = domain::album::vo::Contribution;

pub async fn execute(
  album_repository: &mut impl crate::album::repository::AlbumRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    Input::Create(album) => {
      album_repository
        .create(&album)
        .await
        .map_err(Error::RepositoryError)?;
    }
    Input::Update {
      entity: mut album,
      changes,
    } => {
      album.apply_changes(&changes);
      album_repository
        .update(&album)
        .await
        .map_err(Error::RepositoryError)?;
    }
    Input::Delete(album) => {
      album_repository
        .delete_by_id(album.id())
        .await
        .map_err(Error::RepositoryError)?;
    }
  }

  Ok(())
}
