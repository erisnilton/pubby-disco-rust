use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use serde_json::json;

use crate::{domain::user::User, shared::vo::UUID4, AppState};

#[derive(Debug, Default, Clone)]
pub struct InMemoryUserRepository {
  data: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl InMemoryUserRepository {
  pub fn new(state: &AppState) -> Self {
    Self {
      data: Arc::clone(&state.in_memory),
    }
  }
}

impl crate::domain::user::UserRepository for InMemoryUserRepository {
  async fn create(&mut self, user: User) -> Result<User, crate::domain::user::UserRepositoryError> {
    let mut data = self.data.write().unwrap();

    data.insert(
      user.id.to_string(),
      json!({
        "id": user.id.to_string(),
        "username": user.username.clone(),
        "display_name": user.display_name.clone(),
        "email": user.email.clone(),
        "password": user.password.clone(),
        "is_curator": user.is_curator.clone(),
        "created_at": user.created_at.clone(),
        "updated_at": user.updated_at.clone(),
      }),
    );

    Ok(user)
  }

  async fn find_by_username(
    &mut self,
    username: impl Into<String>,
  ) -> Result<Option<User>, crate::domain::user::UserRepositoryError> {
    let username = username.into();
    let data = self.data.read().unwrap();

    let user = data
      .values()
      .find(|value| value["username"].as_str().unwrap_or_default() == username)
      .map(|value| User {
        id: UUID4::new(value["id"].as_str().unwrap()).unwrap_or_default(),
        created_at: chrono::DateTime::parse_from_rfc3339(value["created_at"].as_str().unwrap())
          .unwrap_or_default()
          .naive_utc(),
        updated_at: chrono::DateTime::parse_from_rfc3339(value["updated_at"].as_str().unwrap())
          .unwrap_or_default()
          .naive_utc(),
        username: value["username"].as_str().unwrap().to_string(),
        display_name: value["display_name"].as_str().unwrap().to_string(),
        email: value["email"].as_str().unwrap().to_string(),
        is_curator: value["is_curator"].as_bool().unwrap(),
        password: value["password"].as_str().unwrap().to_string(),
      });

    Ok(user)
  }

  async fn find_by_id(
    &mut self,
    id: UUID4,
  ) -> Result<Option<User>, crate::domain::user::UserRepositoryError> {
    let data = self.data.read().unwrap();
    let user = data.get(&id.to_string()).map(|value| User {
      id: UUID4::new(value["id"].as_str().unwrap()).unwrap_or_default(),
      created_at: chrono::DateTime::parse_from_rfc3339(value["created_at"].as_str().unwrap())
        .unwrap_or_default()
        .naive_utc(),
      updated_at: chrono::DateTime::parse_from_rfc3339(value["updated_at"].as_str().unwrap())
        .unwrap_or_default()
        .naive_utc(),
      username: value["username"].as_str().unwrap().to_string(),
      display_name: value["display_name"].as_str().unwrap().to_string(),
      email: value["email"].as_str().unwrap().to_string(),
      is_curator: value["is_curator"].as_bool().unwrap(),
      password: value["password"].as_str().unwrap().to_string(),
    });
    Ok(user)
  }
}
