use std::collections::HashSet;

use chrono::{NaiveDate, NaiveDateTime};
use serde_json::json;

use crate::shared::vo::{IntoRecord, UUID4};

use super::AlbumEntity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate, Default)]
pub struct UpdateAlbumDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  #[validate(url)]
  pub cover: Option<String>,

  // todo: change to date
  pub release_date: Option<NaiveDate>,

  #[validate(range(min = 0, max = 18))]
  pub parental_rating: Option<i8>,

  #[validate(length(min = 1))]
  pub artist_ids: Option<HashSet<UUID4>>,
}

#[derive(Debug, serde::Serialize)]
pub struct AlbumPresenter {
  pub id: String,
  pub name: String,
  pub cover: Option<String>,
  pub release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<i8>,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl IntoRecord for UpdateAlbumDto {
  fn into(&self) -> serde_json::Value {
    json!({
      "name": self.name,
      "cover": self.cover,
      "release_date": self.release_date,
      "parental_rating": self.parental_rating,
      "artist_ids": self.artist_ids,
    })
  }
}

impl From<AlbumEntity> for AlbumPresenter {
  fn from(value: AlbumEntity) -> Self {
    Self {
      id: value.id.to_string(),
      name: value.name,
      cover: value.cover,
      release_date: value.release_date,
      parental_rating: value.parental_rating,
      created_at: value.created_at,
      updated_at: value.updated_at,
    }
  }
}

impl From<serde_json::Value> for UpdateAlbumDto {
  fn from(value: serde_json::Value) -> Self {
    let value = value.as_object().unwrap();
    Self {
      name: value
        .get("name")
        .and_then(|v| v.as_str())
        .map(|v| v.to_string()),
      cover: value
        .get("cover")
        .and_then(|v| v.as_str())
        .map(|v| v.to_string()),
      release_date: value
        .get("release_date")
        .and_then(|v| v.as_str())
        .map(|v| NaiveDate::parse_from_str(v, "%Y-%m-%d").unwrap_or_default()),
      parental_rating: value
        .get("parental_rating")
        .and_then(|v| v.as_i64())
        .map(|v| v as i8),
      artist_ids: value.get("artist_ids").and_then(|v| v.as_array()).map(|v| {
        v.iter()
          .map(|v| UUID4::new(v.as_str().unwrap()).unwrap())
          .collect()
      }),
    }
  }
}
