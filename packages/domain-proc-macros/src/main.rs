use domain_proc_macros::Entity;
use syn::token::Use;

#[derive(Entity, Debug)]
struct User {
  id: usize,
  name: String,
  email: String,
  updated_at: chrono::NaiveDateTime,
}

impl Default for User {
  fn default() -> Self {
    Self {
      id: 0,
      name: "".to_string(),
      email: "".to_string(),
      updated_at: chrono::Utc::now().naive_utc(),
    }
  }
}

fn main() {
  let u = UserBuilder::new().id(1).build();
}
