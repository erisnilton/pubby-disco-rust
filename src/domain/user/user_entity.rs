use chrono::Utc;

use crate::shared::vo::UUID4;

#[derive(Debug, Clone)]
pub struct User {
  pub id: UUID4,
  pub username: String,
  pub display_name: String,
  pub email: String,
  pub password: String,
  pub is_curator: bool,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl Default for User {
  fn default() -> Self {
    Self {
      id: UUID4::default(),
      username: String::default(),
      display_name: String::default(),
      email: String::default(),
      password: String::default(),
      is_curator: false,

      created_at: chrono::Utc::now().naive_utc(),
      updated_at: chrono::Utc::now().naive_utc(),
    }
  }
}
