use crate::domain::{album::Album, media::Media};

#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::domain::media::repository::Error),
  AlbumRepositoryError(crate::domain::album::repository::Error),
}

pub type Input = crate::domain::media::contribution::Contribution;

async fn create_single(
  media: &mut Media,
  album_repository: &mut impl crate::domain::album::repository::AlbumRepository,
) -> Result<(), Error> {
  let album = Album::builder()
    .id(media.id().clone())
    .name(media.name().clone())
    .artist_ids(media.interpreter_ids().clone())
    .cover(media.cover().clone())
    .parental_rating(*media.parental_rating())
    .album_type(crate::domain::album::AlbumType::Single)
    .created_at(*media.created_at())
    .updated_at(*media.updated_at())
    .release_date(*media.release_date())
    .build();

  media.album_ids_mut().insert(album.id().clone());

  album_repository
    .create(&album)
    .await
    .map_err(Error::AlbumRepositoryError)?;
  Ok(())
}

async fn delete_single(
  media: &mut Media,
  album_repository: &mut impl crate::domain::album::repository::AlbumRepository,
) -> Result<(), Error> {
  if media.album_ids().contains(&media.id()) {
    let media_id = media.id().clone();
    media.album_ids_mut().remove(&media_id);
    album_repository
      .delete_by_id(&media.id())
      .await
      .map_err(Error::AlbumRepositoryError)?;
  }
  Ok(())
}

pub async fn execute(
  media_repository: &mut impl crate::domain::media::repository::MediaRepository,
  album_repository: &mut impl crate::domain::album::repository::AlbumRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    crate::domain::media::contribution::Contribution::Create(mut media) => {
      if *media.is_single() {
        create_single(&mut media, album_repository).await?;
      }
      media_repository
        .create(&media)
        .await
        .map_err(Error::RepositoryError)?;

      Ok(())
    }
    crate::domain::media::contribution::Contribution::Update {
      entity: mut media,
      changes,
    } => {
      media.apply_changes(&changes);

      match changes.is_single {
        Some(true) => create_single(&mut media, album_repository).await?,
        Some(false) => {
          delete_single(&mut media, album_repository).await?;
        }
        None => {}
      }

      media_repository
        .update(&media)
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
    crate::domain::media::contribution::Contribution::Delete(mut media) => {
      if *media.is_single() {
        delete_single(&mut media, album_repository).await?;
      }
      media_repository
        .delete_by_id(&media.id())
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
  }
}
