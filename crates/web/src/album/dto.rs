use std::collections::HashSet;

use application::album::stories::contribute::CreateInput;
use chrono::NaiveDate;
use domain::album::{entity::AlbumType, vo::changes::Changes};
use shared::vo::{Slug, UUID4};

#[derive(Debug, serde::Deserialize)]
pub struct FindAllQuery {
  pub name: Option<String>,
  pub slug: Option<Slug>,
  pub artist_id: Option<UUID4>,
  pub album_type: Option<AlbumType>,
  pub release_date: Option<chrono::NaiveDate>,
  pub min_release_date: Option<chrono::NaiveDate>,
  pub max_release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub min_parental_rating: Option<u8>,
  pub max_parental_rating: Option<u8>,
  pub search: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct CreateAlbumDTO {
  #[validate(length(min = 1, max = 128))]
  pub name: String,

  #[validate(url)]
  pub cover: Option<String>,

  pub album_type: AlbumType,

  pub release_date: Option<chrono::NaiveDate>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  #[validate(length(min = 1))]
  pub artist_ids: HashSet<UUID4>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate, Default)]
pub struct UpdateAlbumDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  #[validate(url)]
  pub cover: Option<String>,

  pub album_type: Option<AlbumType>,

  pub slug: Option<Slug>,

  pub release_date: Option<NaiveDate>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  #[validate(length(min = 1))]
  pub artist_ids: Option<HashSet<UUID4>>,
}

impl From<CreateAlbumDTO> for CreateInput {
  fn from(value: CreateAlbumDTO) -> Self {
    Self {
      name: value.name,
      cover: value.cover,
      album_type: value.album_type,
      release_date: value.release_date,
      parental_rating: value.parental_rating,
      artist_ids: value.artist_ids,
    }
  }
}

impl From<UpdateAlbumDto> for Changes {
  fn from(value: UpdateAlbumDto) -> Self {
    Self {
      name: value.name,
      album_type: value.album_type,
      cover: value.cover,
      slug: value.slug,
      release_date: value.release_date,
      parental_rating: value.parental_rating,
      artist_ids: value.artist_ids,
    }
  }
}
