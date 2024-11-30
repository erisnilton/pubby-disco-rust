use crate::{
  domain::album::{album_aggregate::AlbumAggregate, Album},
  infra::actix::artist::dto::ArtistPresenter,
  shared::vo::{Slug, UUID4},
};

#[derive(Debug, serde::Serialize)]
pub struct AlbumPresenter {
  id: UUID4,
  name: String,
  slug: Slug,
  cover: Option<String>,
  parental_rating: u8,
  release_date: Option<chrono::NaiveDate>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<Album> for AlbumPresenter {
  fn from(value: Album) -> Self {
    Self {
      id: value.id().clone(),
      name: value.name().clone(),
      cover: value.cover().clone(),
      release_date: *value.release_date(),
      parental_rating: *value.parental_rating(),
      slug: value.slug().clone(),
      created_at: *value.created_at(),
      updated_at: *value.updated_at(),
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct AlbumAggregatePresenter {
  id: UUID4,
  name: String,
  slug: Slug,
  cover: Option<String>,
  parental_rating: u8,
  release_date: Option<chrono::NaiveDate>,
  artists: Vec<ArtistPresenter>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<AlbumAggregate> for AlbumAggregatePresenter {
  fn from(value: AlbumAggregate) -> Self {
    let (album, artists) = value.into_parts();
    Self {
      id: album.id().clone(),
      name: album.name().clone(),
      cover: album.cover().clone(),
      release_date: *album.release_date(),
      parental_rating: *album.parental_rating(),
      slug: album.slug().clone(),
      artists: artists
        .clone()
        .into_iter()
        .map(ArtistPresenter::from)
        .collect(),
      created_at: *album.created_at(),
      updated_at: *album.updated_at(),
    }
  }
}
