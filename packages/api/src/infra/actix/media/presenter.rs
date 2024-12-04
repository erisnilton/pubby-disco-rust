use crate::{
  domain::media::media_aggregate::MediaAggregate,
  shared::{
    paged::Paged,
    vo::{Slug, UUID4},
  },
};

#[derive(Debug, serde::Serialize)]
pub struct MediaAggregatePresenter {
  id: UUID4,
  name: String,
  media_type: crate::domain::media::MediaType,
  is_single: bool,
  slug: Slug,
  release_date: Option<chrono::NaiveDate>,
  cover: Option<String>,
  parental_rating: u8,
  composers: Vec<crate::infra::actix::artist::presenter::ArtistPresenter>,
  interpreters: Vec<crate::infra::actix::artist::presenter::ArtistPresenter>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<crate::domain::media::media_aggregate::MediaAggregate> for MediaAggregatePresenter {
  fn from(value: crate::domain::media::media_aggregate::MediaAggregate) -> Self {
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
        .map(crate::infra::actix::artist::presenter::ArtistPresenter::from)
        .collect(),
      interpreters: value
        .interpreters
        .into_iter()
        .map(crate::infra::actix::artist::presenter::ArtistPresenter::from)
        .collect(),
      is_single: *value.media.is_single(),
      created_at: *value.media.created_at(),
      updated_at: *value.media.updated_at(),
    }
  }
}

impl From<Paged<MediaAggregate>> for Paged<MediaAggregatePresenter> {
  fn from(value: Paged<MediaAggregate>) -> Self {
    Self {
      items: value
        .items
        .into_iter()
        .map(MediaAggregatePresenter::from)
        .collect(),
      total_items: value.total_items,
      total_pages: value.total_pages,
      page: value.page,
    }
  }
}
