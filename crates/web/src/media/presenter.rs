use domain::media::{aggregate::MediaAggregate, entity::MediaType};
use shared::vo::{Slug, UUID4};

use crate::artist::presenter::ArtistPresenter;

#[derive(Debug, serde::Serialize)]
pub struct MediaAggregatePresenter {
  id: UUID4,
  name: String,
  media_type: MediaType,
  is_single: bool,
  slug: Slug,
  release_date: Option<chrono::NaiveDate>,
  cover: Option<String>,
  parental_rating: u8,
  composers: Vec<ArtistPresenter>,
  interpreters: Vec<ArtistPresenter>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<MediaAggregate> for MediaAggregatePresenter {
  fn from(value: MediaAggregate) -> Self {
    Self {
      id: value.media.id().clone(),
      name: value.media.name().clone(),
      media_type: value.media.media_type().clone(),
      slug: value.media.slug().clone(),
      release_date: *value.media.release_date(),
      cover: value.media.cover().clone(),
      parental_rating: *value.media.parental_rating(),
      composers: value
        .composers
        .into_iter()
        .map(ArtistPresenter::from)
        .collect(),
      interpreters: value
        .interpreters
        .into_iter()
        .map(ArtistPresenter::from)
        .collect(),
      is_single: *value.media.is_single(),
      created_at: *value.media.created_at(),
      updated_at: *value.media.updated_at(),
    }
  }
}
