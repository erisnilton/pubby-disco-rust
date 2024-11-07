use chrono::Utc;
use serde_json::json;

use crate::shared::vo::{IntoRecord, Slug, UUID4};

use super::dto::UpdateArtistDto;

#[derive(Debug, Clone, PartialEq)]
pub struct ArtistPresenter {
  pub id: String,
  pub slug: String,
  pub name: String,
  pub country: String,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Artist {
  pub id: UUID4,
  pub slug: Slug,
  pub name: String,
  pub country: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl Artist {
  pub fn new(name: String, slug: Slug, country: String) -> Self {
    Self {
      id: UUID4::generate(),
      slug,
      name,
      country,
      ..Default::default()
    }
  }
  pub fn apply_changes(&mut self, old_value: &UpdateArtistDto, new_value: &UpdateArtistDto) {
    if old_value.name != new_value.name {
      self.name = new_value.name.clone().unwrap_or_default();
    }

    if old_value.slug != new_value.slug {
      self.slug = Slug::new(&new_value.slug.clone().unwrap_or_default()).unwrap_or_default();
    }
    if old_value.country != new_value.country {
      self.country = new_value.country.clone().unwrap_or_default();
    }
  }
}

impl Default for Artist {
  fn default() -> Self {
    Self {
      id: UUID4::generate(),
      slug: Slug::default(),
      name: "".to_string(),
      country: "".to_string(),
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
    }
  }
}

impl IntoRecord for Artist {
  fn into(&self) -> serde_json::Value {
    return json!({
      "id": self.id.0.clone(),
      "slug": self.slug.to_string(),
      "name": self.name,
      "country": self.country,
      "created_at": self.created_at,
      "updated_at": self.updated_at,
    });
  }
}

impl From<serde_json::Value> for Artist {
  fn from(value: serde_json::Value) -> Self {
    Artist {
      id: UUID4::new(value["id"].as_str().unwrap_or_default()).unwrap_or_default(),
      name: value["name"].as_str().unwrap_or_default().to_string(),
      slug: Slug::new(value["slug"].as_str().unwrap_or_default()).unwrap_or_default(),
      country: value["country"].as_str().unwrap_or_default().to_string(),
      created_at: value["created_at"]
        .as_str()
        .unwrap_or_default()
        .parse()
        .unwrap(),
      updated_at: value["updated_at"]
        .as_str()
        .unwrap_or_default()
        .parse()
        .unwrap(),
    }
  }
}
