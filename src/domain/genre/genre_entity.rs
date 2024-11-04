use chrono::Utc;
use serde_json::json;
use validator::{Validate, ValidationErrors};

use crate::shared::vo::{GetChanges, IntoRecord, Slug, UUID4};

use super::dto::UpdateGenreDto;

#[derive(Debug, Clone, PartialEq)]
pub struct Genre {
  pub id: UUID4,
  pub slug: Slug,
  pub name: String,
  pub parent_id: Option<UUID4>,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl Genre {
  pub fn new(input: GenreInput) -> Result<Genre, ValidationErrors> {
    input.validate()?;

    Ok(Genre {
      id: input.id,
      name: input.name,
      slug: input.slug,
      parent_id: input.parent_id,
      created_at: input.created_at,
      updated_at: input.updated_at,
    })
  }

  pub fn apply_changes(&mut self, old_value: &UpdateGenreDto, new_value: &UpdateGenreDto) {
    if old_value.name != new_value.name {
      self.name = new_value.name.clone().unwrap_or_default();
    }

    if old_value.slug != new_value.slug {
      self.slug =
        Slug::new(new_value.slug.clone().unwrap_or_default().as_str()).unwrap_or_default();
    }
    if old_value.parent_id != new_value.parent_id {
      self.parent_id = new_value
        .parent_id
        .clone()
        .map(|id| UUID4::new(id).unwrap_or_default());
    }
  }
}

impl Default for Genre {
  fn default() -> Self {
    Self {
      id: UUID4::default(),
      slug: Slug::default(),
      name: "".to_string(),
      parent_id: None,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }
}

impl IntoRecord for Genre {
  fn into(&self) -> serde_json::Value {
    return json!({
      "id": self.id.0.clone(),
      "name": self.name.clone(),
      "slug": self.slug.0.clone(),
      "parent_id": self.parent_id.clone(),
      "created_at": self.created_at.clone(),
      "updated_at": self.updated_at.clone()
    });
  }
}

impl From<serde_json::Value> for Genre {
  fn from(value: serde_json::Value) -> Self {
    Genre {
      id: UUID4::new(value["id"].as_str().unwrap_or_default()).unwrap_or_default(),
      name: value["name"].as_str().unwrap_or_default().to_string(),
      slug: Slug::new(value["slug"].as_str().unwrap_or_default()).unwrap_or_default(),
      parent_id: value["parent_id"]
        .as_str()
        .map(|id| UUID4::new(id).unwrap_or_default()),
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

impl GetChanges<UpdateGenreDto> for Genre {
  fn get_changes(&self, changes: UpdateGenreDto) -> (UpdateGenreDto, UpdateGenreDto) {
    let mut old_value = UpdateGenreDto::default();
    let mut new_value = UpdateGenreDto::default();

    if changes.name.is_some() && changes.name != Some(self.name.clone()) {
      old_value.name = Some(self.name.clone());
      new_value.name = changes.name;
    }

    if changes.slug.is_some() && changes.slug != Some(self.slug.to_string()) {
      old_value.slug = Some(self.slug.to_string());
      new_value.slug = changes.slug;
    }

    if changes.parent_id.is_some()
      && changes.parent_id != self.parent_id.as_ref().map(|id| id.0.clone())
    {
      old_value.parent_id = self.parent_id.as_ref().map(|id| id.0.clone());
      new_value.parent_id = changes.parent_id;
    }

    (old_value, new_value)
  }
}

#[derive(Debug, Validate)]
pub struct GenreInput {
  pub id: UUID4,
  pub slug: Slug,

  #[validate(length(min = 1, max = 128))]
  pub name: String,

  pub parent_id: Option<UUID4>,

  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl Default for GenreInput {
  fn default() -> Self {
    Self {
      id: UUID4::default(),
      name: String::default(),
      slug: Slug::default(),
      parent_id: None,
      created_at: chrono::Utc::now(),
      updated_at: chrono::Utc::now(),
    }
  }
}

impl TryInto<Genre> for GenreInput {
  type Error = ValidationErrors;

  fn try_into(self) -> Result<Genre, Self::Error> {
    Genre::new(self)
  }
}
