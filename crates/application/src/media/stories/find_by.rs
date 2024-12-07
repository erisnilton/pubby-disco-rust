use domain::media::{aggregate::MediaAggregate, entity::MediaType};
use shared::{
  paged::{Paged, RequestPageParams},
  vo::{Slug, UUID4},
};

use std::collections::HashSet;

use crate::media::repository::{FindByQuery, MediaRepository};

#[derive(Debug, Clone)]
pub struct Input {
  pub page: RequestPageParams,
  pub search: Option<String>,
  pub release_date: Option<chrono::NaiveDate>,
  pub min_release_date: Option<chrono::NaiveDate>,
  pub max_release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub min_parental_rating: Option<u8>,
  pub max_parental_rating: Option<u8>,
  pub is_single: Option<bool>,
  pub media_type: Option<MediaType>,
  pub slug: Option<Slug>,
  pub artist_ids: Option<HashSet<UUID4>>,
  pub composer_ids: Option<HashSet<UUID4>>,
  pub genre_ids: Option<HashSet<UUID4>>,
  pub album_ids: Option<HashSet<UUID4>>,
}

pub enum Error {
  RepositoryError(crate::media::repository::Error),
}

pub async fn execute(
  media_repository: &mut impl MediaRepository,
  input: Input,
) -> Result<Paged<MediaAggregate>, Error> {
  let result = media_repository
    .find_by(&FindByQuery {
      page: input.page,
      release_date: input.release_date,
      min_release_date: input.min_release_date,
      max_release_date: input.max_release_date,
      parental_rating: input.parental_rating,
      min_parental_rating: input.min_parental_rating,
      max_parental_rating: input.max_parental_rating,
      is_single: input.is_single,
      media_type: input.media_type,
      slug: input.slug,
      artist_ids: input.artist_ids,
      composer_ids: input.composer_ids,
      genre_ids: input.genre_ids,
      album_ids: input.album_ids,
      search: input.search,
    })
    .await
    .map_err(Error::RepositoryError)?;

  Ok(result)
}
