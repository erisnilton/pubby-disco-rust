use std::collections::HashSet;

use chrono::NaiveDate;

use crate::{
  domain, infra,
  shared::vo::{Slug, UUID4},
};

#[derive(Debug, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct CreateAlbumDTO {
  #[validate(length(min = 1, max = 128))]
  pub name: String,

  #[validate(url)]
  pub cover: Option<String>,

  pub album_type: domain::album::AlbumType,

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

  pub album_type: Option<domain::album::AlbumType>,

  pub slug: Option<Slug>,

  pub release_date: Option<NaiveDate>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  #[validate(length(min = 1))]
  pub artist_ids: Option<HashSet<UUID4>>,
}

impl From<infra::actix::album::dto::CreateAlbumDTO>
  for domain::album::stories::contribute::CreateInput
{
  fn from(value: infra::actix::album::dto::CreateAlbumDTO) -> Self {
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

impl From<UpdateAlbumDto> for domain::album::contribution::changes::Changes {
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
