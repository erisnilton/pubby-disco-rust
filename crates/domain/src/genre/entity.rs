use entity::Entity;

#[derive(Debug, Entity, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Genre {
  id: shared::vo::UUID4,
  slug: shared::vo::Slug,
  name: String,
  parent_id: Option<shared::vo::UUID4>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Genre {
  pub fn apply_changes(&mut self, changes: &crate::genre::vo::Changes) {
    if let Some(value) = &changes.name {
      self.name = value.clone();
    }

    if let Some(value) = &changes.slug {
      self.slug = value.clone();
    }

    if let Some(value) = &changes.parent_id {
      self.parent_id = value.clone();
    }

    self.updated_at = shared::util::naive_now();
  }
}

impl Default for Genre {
  fn default() -> Self {
    let now = shared::util::naive_now();
    Self {
      id: shared::vo::UUID4::default(),
      slug: shared::vo::Slug::default(),
      name: "".to_string(),
      parent_id: None,
      created_at: now,
      updated_at: now,
    }
  }
}
