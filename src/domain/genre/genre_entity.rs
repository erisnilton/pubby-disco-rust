use derive_builder::Builder;

#[derive(Debug, Builder, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Genre {
  id: crate::shared::vo::UUID4,
  slug: crate::shared::vo::Slug,
  name: String,
  parent_id: Option<crate::shared::vo::UUID4>,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Genre {
  pub fn apply_changes(&mut self, changes: &crate::domain::genre::contribution::changes::Changes) {
    if let Some(value) = &changes.name {
      self.set_name(value.clone());
    }
    if let Some(value) = &changes.slug {
      self.set_slug(value.clone());
    }
    if let Some(value) = &changes.parent_id {
      self.set_parent_id(value.clone());
    }
  }

  pub fn id(&self) -> crate::shared::vo::UUID4 {
    self.id.clone()
  }

  pub fn slug(&self) -> crate::shared::vo::Slug {
    self.slug.clone()
  }

  pub fn set_slug(&mut self, slug: crate::shared::vo::Slug) {
    self.slug = slug;
    self.update_entity();
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
    self.update_entity();
  }

  pub fn parent_id(&self) -> Option<crate::shared::vo::UUID4> {
    self.parent_id.clone()
  }

  pub fn set_parent_id(&mut self, parent_id: Option<crate::shared::vo::UUID4>) {
    self.parent_id = parent_id;
    self.update_entity();
  }

  pub fn created_at(&self) -> chrono::NaiveDateTime {
    self.created_at.clone()
  }

  pub fn updated_at(&self) -> chrono::NaiveDateTime {
    self.updated_at.clone()
  }

  pub fn update_entity(&mut self) {
    self.updated_at = crate::shared::util::naive_now();
  }
}

impl Default for Genre {
  fn default() -> Self {
    let now = crate::shared::util::naive_now();
    Self {
      id: crate::shared::vo::UUID4::default(),
      slug: crate::shared::vo::Slug::default(),
      name: "".to_string(),
      parent_id: None,
      created_at: now,
      updated_at: now,
    }
  }
}

impl From<serde_json::Value> for Genre {
  fn from(value: serde_json::Value) -> Self {
    Genre {
      id: crate::shared::vo::UUID4::new(value["id"].as_str().unwrap_or_default())
        .unwrap_or_default(),
      name: value["name"].as_str().unwrap_or_default().to_string(),
      slug: crate::shared::vo::Slug::new(value["slug"].as_str().unwrap_or_default())
        .unwrap_or_default(),
      parent_id: value["parent_id"]
        .as_str()
        .map(|id| crate::shared::vo::UUID4::new(id).unwrap_or_default()),
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
