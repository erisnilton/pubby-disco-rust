use domain::album::{aggregate::AlbumAggregate, entity::AlbumType};
use shared::{
  paged::{Paged, RequestPageParams},
  vo::{Slug, UUID4},
};

use crate::album::repository::{AlbumRepository, FindAllQuery};

pub enum Error {
  RepositoryError(crate::album::repository::Error),
}
pub struct Input {
  pub page: RequestPageParams,
  pub name: Option<String>,
  pub slug: Option<Slug>,
  pub artist_ids: Option<Vec<UUID4>>,
  pub album_type: Option<AlbumType>,
  pub release_date: Option<chrono::NaiveDate>,
  pub min_release_date: Option<chrono::NaiveDate>,
  pub max_release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub min_parental_rating: Option<u8>,
  pub max_parental_rating: Option<u8>,
  pub search: Option<String>,
}
pub async fn execute(
  album_repository: &mut impl AlbumRepository,
  input: Input,
) -> Result<Paged<AlbumAggregate>, Error> {
  let result = album_repository
    .find_by(&FindAllQuery {
      page: input.page,
      name: input.name,
      slug: input.slug,
      artist_ids: input.artist_ids,
      album_type: input.album_type,
      release_date: input.release_date,
      min_release_date: input.min_release_date,
      max_release_date: input.max_release_date,
      parental_rating: input.parental_rating,
      min_parental_rating: input.min_parental_rating,
      max_parental_rating: input.max_parental_rating,
      search: input.search,
    })
    .await
    .map_err(Error::RepositoryError)?;

  Ok(result)
}
