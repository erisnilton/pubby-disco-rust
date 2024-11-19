use crate::shared::util::naive_now;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
  pub id: crate::shared::vo::UUID4,
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
    let now = naive_now();
    Self {
      id: crate::shared::vo::UUID4::default(),

      username: String::default(),
      display_name: String::default(),
      email: String::default(),
      password: String::default(),
      is_curator: false,

      created_at: now,
      updated_at: now,
    }
  }
}
