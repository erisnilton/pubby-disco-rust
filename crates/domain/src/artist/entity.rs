use entity::Entity;

use shared::util::naive_now;

#[derive(Debug, Clone, Entity, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Artist {
  id: shared::vo::UUID4,
  slug: shared::vo::Slug,
  name: String,
  country: Option<String>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Artist {
  pub fn apply_changes(&mut self, changes: &super::vo::changes::Changes) {
    if let Some(value) = &changes.name {
      self.name = value.clone();
    }

    if let Some(value) = &changes.slug {
      self.slug = value.clone();
    }

    if let Some(value) = &changes.country {
      self.country = Some(value.clone());
    }

    self.updated_at = naive_now();
  }
}

impl Default for Artist {
  fn default() -> Self {
    let now = naive_now();

    Self {
      id: shared::vo::UUID4::default(),
      slug: shared::vo::Slug::default(),
      name: "".to_string(),
      country: None,
      created_at: now,
      updated_at: now,
    }
  }
}
