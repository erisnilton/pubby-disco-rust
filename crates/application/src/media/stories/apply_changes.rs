use domain::{
  album::entity::{Album, AlbumType},
  media::entity::Media,
};

use crate::{album::repository::AlbumRepository, media::repository::MediaRepository};

#[derive(Debug, Clone)]
pub enum Error {
  RepositoryError(crate::media::repository::Error),
  AlbumRepositoryError(crate::album::repository::Error),
}

pub type Input = domain::media::vo::Contribution;

async fn create_single(
  media: &mut Media,
  album_repository: &mut impl AlbumRepository,
) -> Result<(), Error> {
  let album = Album::builder()
    .id(media.id().clone())
    .name(media.name().clone())
    .artist_ids(media.interpreter_ids().clone())
    .cover(media.cover().clone())
    .parental_rating(*media.parental_rating())
    .album_type(AlbumType::Single)
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
  album_repository: &mut impl AlbumRepository,
) -> Result<(), Error> {
  if media.album_ids().contains(media.id()) {
    let media_id = media.id().clone();
    media.album_ids_mut().remove(&media_id);
    album_repository
      .delete_by_id(media.id())
      .await
      .map_err(Error::AlbumRepositoryError)?;
  }
  Ok(())
}

pub async fn execute(
  media_repository: &mut impl MediaRepository,
  album_repository: &mut impl AlbumRepository,
  input: Input,
) -> Result<(), Error> {
  match input {
    Input::Create(mut media) => {
      if *media.is_single() {
        create_single(&mut media, album_repository).await?;
      }
      media_repository
        .create(&media)
        .await
        .map_err(Error::RepositoryError)?;

      Ok(())
    }
    Input::Update {
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
    Input::Delete(mut media) => {
      if *media.is_single() {
        delete_single(&mut media, album_repository).await?;
      }
      media_repository
        .delete_by_id(media.id())
        .await
        .map_err(Error::RepositoryError)?;
      Ok(())
    }
  }
}
