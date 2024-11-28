pub enum Error {
  RepositoryError(crate::domain::genre::repository::Error),
  GenreNotFound,
}

pub async fn execute(
  genre_repository: &mut impl crate::domain::genre::GenreRepository,
  slug: crate::shared::vo::Slug,
) -> Result<crate::domain::genre::GenreAggregate, Error> {
  let genre_aggregate = genre_repository
    .find_genre_and_subgenre_by_slug(&slug)
    .await
    .map_err(Error::RepositoryError)?
    .ok_or(Error::GenreNotFound)?;

  Ok(genre_aggregate)
}
