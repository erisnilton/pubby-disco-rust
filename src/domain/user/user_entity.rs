use chrono::Utc;

use crate::shared::vo::UUID4;

#[derive(Debug, Clone)]
pub struct User {
  pub id: UUID4,
  pub name: String,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl Default for User {
  fn default() -> Self {
    Self {
      id: UUID4::default(),
      name: String::default(),
      created_at: chrono::Utc::now(),
      updated_at: chrono::Utc::now(),
    }
  }
}
