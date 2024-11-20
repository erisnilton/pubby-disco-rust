use crate::shared::util::naive_now;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Artist {
  pub id: crate::shared::vo::UUID4,

  pub slug: crate::shared::vo::Slug,
  pub name: String,
  pub country: Option<String>,

  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl Artist {
  pub fn apply_changes(&mut self, changes: &super::contribution::changes::Changes) {
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
      id: crate::shared::vo::UUID4::default(),
      slug: crate::shared::vo::Slug::default(),
      name: "".to_string(),
      country: None,
      created_at: now,
      updated_at: now,
    }
  }
}
