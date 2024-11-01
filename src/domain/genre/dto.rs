use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::shared::vo::IntoRecord;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateGenreDto {
  #[validate(length(min = 1, max = 128))]
  pub name: String,

  #[validate(custom(function = "crate::shared::validator::uuid"))]
  pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct UpdateGenreDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  #[validate(length(min = 1, max = 128))]
  pub slug: Option<String>,

  #[validate(custom(function = "crate::shared::validator::uuid"))]
  pub parent_id: Option<String>,
}

impl IntoRecord for UpdateGenreDto {
  fn into(&self) -> serde_json::Value {
    json!({
      "name": self.name,
      "slug": self.slug,
      "parent_id": self.parent_id
    })
  }
}

impl From<serde_json::Value> for UpdateGenreDto {
  fn from(value: serde_json::Value) -> Self {
    Self {
      name: value["name"].as_str().map(|s| s.to_string()),
      slug: value["slug"].as_str().map(|s| s.to_string()),
      parent_id: value["parent_id"].as_str().map(|s| s.to_string()),
    }
  }
}
