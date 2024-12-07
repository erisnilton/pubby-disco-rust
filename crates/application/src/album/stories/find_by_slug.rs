use domain::album::aggregate::AlbumAggregate;
use shared::vo::Slug;

use crate::album::repository::AlbumRepository;

pub enum Error {
  RepositoryError(crate::album::repository::Error),
  AlbumNotFound,
}

pub async fn execute(
  album_repository: &mut impl AlbumRepository,
  slug: &Slug,
  artist_slug: &Slug,
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
