use shared::vo::Slug;

use crate::*;

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateMediaDTO {
  #[validate(length(min = 1, max = 128))]
  pub name: String,

  pub slug: Option<crate::shared::vo::Slug>,

  pub media_type: crate::domain::media::MediaType,

  pub release_date: Option<chrono::NaiveDate>,

  #[validate(url)]
  pub cover: Option<String>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  pub genre_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub composer_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub interpreter_ids: std::collections::HashSet<crate::shared::vo::UUID4>,

  pub album_ids: std::collections::HashSet<crate::shared::vo::UUID4>,

  pub is_single: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate, Default)]
pub struct UpdateMediaDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  pub media_type: Option<crate::domain::media::MediaType>,

  pub slug: Option<crate::shared::vo::Slug>,

  pub release_date: Option<chrono::NaiveDate>,

  #[validate(url)]
  pub cover: Option<Option<String>>,

  pub genre_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  pub composer_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub interpreter_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub album_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub is_single: Option<bool>,
}

impl From<CreateMediaDTO> for crate::domain::media::stories::contribute::CreateMediaInput {
  fn from(value: infra::actix::media::dto::CreateMediaDTO) -> Self {
    Self {
      name: value.name,
      slug: value.slug,
      media_type: value.media_type,
      release_date: value.release_date,
      cover: value.cover,
      parental_rating: value.parental_rating,
      genre_ids: value.genre_ids,
      composer_ids: value.composer_ids,
      interpreter_ids: value.interpreter_ids,
      album_ids: value.album_ids,
      is_single: value.is_single,
    }
  }
}

impl From<UpdateMediaDto> for crate::domain::media::contribution::changes::Changes {
  fn from(value: UpdateMediaDto) -> Self {
    Self {
      name: value.name,
      media_type: value.media_type,
      slug: value.slug,
      release_date: value.release_date,
      cover: value.cover,
      genre_ids: value.genre_ids,
      parental_rating: value.parental_rating,
      composer_ids: value.composer_ids,
      interpreter_ids: value.interpreter_ids,
      album_ids: value.album_ids,
      is_single: value.is_single,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct MediaPresenter {
  id: crate::shared::vo::UUID4,
  name: String,
  media_type: domain::media::MediaType,
  slug: Slug,
  release_date: Option<chrono::NaiveDate>,
  cover: Option<String>,
  parental_rating: u8,
  genre_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  composer_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  interpreter_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  album_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  is_single: bool,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<domain::media::Media> for MediaPresenter {
  fn from(value: domain::media::Media) -> Self {
    Self {
      id: value.id().clone(),
      name: value.name().clone(),
      media_type: value.media_type().clone(),
      slug: value.slug().clone(),
      release_date: value.release_date().clone(),
      cover: value.cover().clone(),
      parental_rating: value.parental_rating().clone(),
      genre_ids: value.genre_ids().clone(),
      composer_ids: value.composer_ids().clone(),
      interpreter_ids: value.interpreter_ids().clone(),
      album_ids: value.album_ids().clone(),
      is_single: value.is_single().clone(),
      created_at: value.created_at().clone(),
      updated_at: value.updated_at().clone(),
    }
  }
}
