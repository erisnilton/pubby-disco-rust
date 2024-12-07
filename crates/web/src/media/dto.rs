use application::media::stories::contribute::CreateMediaInput;
use domain::media::entity::{Media, MediaType};
use shared::vo::{Slug, UUID4};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FindByQuery {
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
  pub artist_id: Option<UUID4>,
  pub composer_id: Option<UUID4>,
  pub genre_id: Option<UUID4>,
  pub album_id: Option<UUID4>,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateMediaDTO {
  #[validate(length(min = 1, max = 128))]
  pub name: String,

  pub slug: Option<Slug>,

  pub media_type: MediaType,

  pub release_date: Option<chrono::NaiveDate>,

  #[validate(url)]
  pub cover: Option<String>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  pub genre_ids: Option<std::collections::HashSet<UUID4>>,

  pub composer_ids: Option<std::collections::HashSet<UUID4>>,

  pub interpreter_ids: std::collections::HashSet<UUID4>,

  pub album_ids: std::collections::HashSet<UUID4>,

  pub is_single: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate, Default)]
pub struct UpdateMediaDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  pub media_type: Option<MediaType>,

  pub slug: Option<Slug>,

  pub release_date: Option<chrono::NaiveDate>,

  #[validate(url)]
  pub cover: Option<Option<String>>,

  pub genre_ids: Option<std::collections::HashSet<UUID4>>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<u8>,

  pub composer_ids: Option<std::collections::HashSet<UUID4>>,

  pub interpreter_ids: Option<std::collections::HashSet<UUID4>>,

  pub album_ids: Option<std::collections::HashSet<UUID4>>,

  pub is_single: Option<bool>,
}

impl From<CreateMediaDTO> for CreateMediaInput {
  fn from(value: CreateMediaDTO) -> Self {
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

impl From<UpdateMediaDto> for domain::media::vo::Changes {
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
  id: UUID4,
  name: String,
  media_type: MediaType,
  slug: Slug,
  release_date: Option<chrono::NaiveDate>,
  cover: Option<String>,
  parental_rating: u8,
  genre_ids: std::collections::HashSet<UUID4>,
  composer_ids: std::collections::HashSet<UUID4>,
  interpreter_ids: std::collections::HashSet<UUID4>,
  album_ids: std::collections::HashSet<UUID4>,
  is_single: bool,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<Media> for MediaPresenter {
  fn from(value: Media) -> Self {
    Self {
      id: value.id().clone(),
      name: value.name().clone(),
      media_type: value.media_type().clone(),
      slug: value.slug().clone(),
      release_date: *value.release_date(),
      cover: value.cover().clone(),
      parental_rating: *value.parental_rating(),
      genre_ids: value.genre_ids().clone(),
      composer_ids: value.composer_ids().clone(),
      interpreter_ids: value.interpreter_ids().clone(),
      album_ids: value.album_ids().clone(),
      is_single: *value.is_single(),
      created_at: *value.created_at(),
      updated_at: *value.updated_at(),
    }
  }
}
