use crate::domain::{
  self,
  album::{album_aggregate::AlbumAggregate, repository::AlbumRepository},
};

pub enum Error {
  RepositoryError(domain::album::repository::Error),
  AlbumNotFound,
}

pub async fn execute(
  album_repository: &mut impl AlbumRepository,
  slug: &crate::shared::vo::Slug,
  artist_slug: &crate::shared::vo::Slug,
) -> Result<AlbumAggregate, Error> {
  let album_aggregate = album_repository
    .find_by_slug(slug, artist_slug)
    .await
    .map_err(Error::RepositoryError)?;

  if let Some(album_aggregate) = album_aggregate {
    return Ok(album_aggregate);
  }

  Err(Error::AlbumNotFound)
}
