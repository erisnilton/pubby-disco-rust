use entity::Entity;

use shared::util::naive_now;

#[derive(Entity, Debug, Clone, PartialEq)]
pub struct User {
  id: shared::vo::UUID4,

  username: String,
  display_name: String,
  email: String,
  password: String,
  is_curator: bool,

  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Default for User {
  fn default() -> Self {
    let now = naive_now();
    Self {
      id: shared::vo::UUID4::default(),

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
