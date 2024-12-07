use domain::genre::aggregate::GenreAggregate;
use shared::vo::Slug;

use crate::genre::repository::GenreRepository;

pub enum Error {
  RepositoryError(crate::genre::repository::Error),
  GenreNotFound,
}

pub async fn execute(
  genre_repository: &mut impl GenreRepository,
  slug: Slug,
) -> Result<GenreAggregate, Error> {
  let genre_aggregate = genre_repository
    .find_genre_and_subgenre_by_slug(&slug)
    .await
    .map_err(Error::RepositoryError)?
    .ok_or(Error::GenreNotFound)?;

  Ok(genre_aggregate)
}
