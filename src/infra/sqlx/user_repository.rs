use uuid::Uuid;

use crate::{
  domain::user::{User, UserRepository, UserRepositoryError},
  AppState,
};

pub struct SqlxUserRepository {
  pool: sqlx::PgPool,
}

impl SqlxUserRepository {
  pub fn new(state: &AppState) -> Self {
    Self {
      pool: state.db.clone(),
    }
  }
}

#[derive(sqlx::FromRow)]
struct UserRecord {
  id: Uuid,
  username: String,
  password: String,
  email: String,
  display_name: String,
  is_curator: bool,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl From<User> for UserRecord {
  fn from(val: User) -> Self {
    UserRecord {
      id: Uuid::parse_str(&val.id.0).unwrap(),
      username: val.username,
      password: val.password,
      email: val.email,
      is_curator: val.is_curator,
      display_name: val.display_name,
      created_at: val.created_at,
      updated_at: val.updated_at,
    }
  }
}

impl From<UserRecord> for User {
  fn from(val: UserRecord) -> Self {
    User {
      id: crate::shared::vo::UUID4(val.id.to_string()),
      username: val.username,
      password: val.password,
      email: val.email,
      is_curator: val.is_curator,
      display_name: val.display_name,
      created_at: val.created_at,
      updated_at: val.updated_at,
    }
  }
}

impl UserRepository for SqlxUserRepository {
  async fn create(
    &mut self,
    user: crate::domain::user::User,
  ) -> Result<crate::domain::user::User, crate::domain::user::UserRepositoryError> {
    let user_record: UserRecord = user.clone().into();

    sqlx::query!(
      r#"
      INSERT INTO "users" ("id", "username", "password", "email", "display_name")
      VALUES ($1, $2, $3, $4, $5)
    "#r,
      user_record.id,
      user_record.username,
      user_record.password,
      user_record.email,
      user_record.display_name
    )
    .execute(&self.pool)
    .await
    .map_err(|e| UserRepositoryError::InternalServerError(e.to_string()))?;

    Ok(user)
  }

  async fn find_by_username(
    &mut self,
    username: impl Into<String>,
  ) -> Result<Option<crate::domain::user::User>, crate::domain::user::UserRepositoryError> {
    let username = username.into();

    let user_record = sqlx::query_as!(
      UserRecord,
      r#"SELECT * FROM "users" WHERE "username" = $1"#,
      username
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| UserRepositoryError::InternalServerError(e.to_string()))?;

    Ok(user_record.map(|record| record.into()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::AppState;

  const TEST_EMAIL: &str = "user@test.com";

  async fn drop_old_data() {
    let db = AppState::default().await.db;

    // Delete old data
    sqlx::query!(r#"DELETE FROM "users" WHERE "email" = $1"#r, TEST_EMAIL)
      .execute(&db)
      .await
      .unwrap();
  }

  #[tokio::test]
  async fn should_create_user() {
    // Load .env file
    dotenvy::dotenv().ok();

    drop_old_data().await;

    let state = AppState::default().await;

    let mut user_repository = SqlxUserRepository::new(&state);

    let username = String::from("test");
    let password = String::from("test");
    let email = String::from(TEST_EMAIL);
    let display_name = String::from("Test User");

    let user = user_repository
      .create(User {
        username: username.clone(),
        password: password.clone(),
        email: email.clone(),
        display_name: display_name.clone(),
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(user.username, username);
    assert_eq!(user.email, email);
    assert_eq!(user.display_name, display_name);

    drop_old_data().await;
  }

  #[tokio::test]
  async fn should_find_user_by_username() {
    // Load .env file
    dotenvy::dotenv().ok();

    let state = AppState::default().await;

    drop_old_data().await;

    let mut user_repository = SqlxUserRepository::new(&state);

    let username = String::from("test");
    let password = String::from("test");
    let email = String::from(TEST_EMAIL);
    let display_name = String::from("Test User");

    user_repository
      .create(User {
        username: username.clone(),
        password: password.clone(),
        email: email.clone(),
        display_name: display_name.clone(),
        ..Default::default()
      })
      .await
      .unwrap();

    let user = user_repository
      .find_by_username(username.clone())
      .await
      .unwrap()
      .unwrap();

    assert_eq!(user.username, username);
    assert_eq!(user.email, email);
    assert_eq!(user.display_name, display_name);

    drop_old_data().await;
  }

  #[tokio::test]
  async fn should_not_find_user_by_username() {
    // Load .env file
    dotenvy::dotenv().ok();

    let state: AppState = AppState::default().await;

    drop_old_data().await;

    let mut user_repository = SqlxUserRepository::new(&state);

    let username = String::from("test");

    let user = user_repository
      .find_by_username(username.clone())
      .await
      .unwrap();

    assert!(user.is_none());

    drop_old_data().await;
  }
}
