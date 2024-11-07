use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::shared::vo::IntoRecord;

use super::Artist;

#[derive(Debug, serde::Serialize)]
pub struct ArtistPresenter {
  pub id: String,
  pub name: String,
  pub slug: String,
  pub country: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ArtistPresenterDTO {
  pub id: String,
  pub slug: String,
  pub name: String,
  pub country: String,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateArtistDto {
  #[validate(length(min = 1, max = 128))]
  pub name: String,

  #[validate(length(min = 1, max = 128))]
  pub slug: String,

  #[validate(length(min = 1, max = 128))]
  pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct UpdateArtistDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  #[validate(length(min = 1, max = 128))]
  pub slug: Option<String>,

  #[validate(length(min = 1, max = 128))]
  pub country: Option<String>,
}

impl From<Artist> for ArtistPresenterDTO {
  fn from(artist: Artist) -> Self {
    Self {
      id: format!("{}", artist.id),
      slug: artist.slug.to_string(),
      name: artist.name,
      country: artist.country,
      created_at: artist.created_at.and_utc(),
      updated_at: artist.updated_at.and_utc(),
    }
  }
}

impl IntoRecord for UpdateArtistDto {
  fn into(&self) -> serde_json::Value {
    json!({
      "name": self.name,
      "slug": self.slug,
      "country": self.country,
    })
  }
}

impl From<serde_json::Value> for UpdateArtistDto {
  fn from(value: serde_json::Value) -> Self {
    Self {
      name: value["name"].as_str().map(|s| s.to_string()),
      slug: value["slug"].as_str().map(|s| s.to_string()),
      country: value["country"].as_str().map(|s| s.to_string()),
    }
  }
}
