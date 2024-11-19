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
    let username: String = username.into();

    let user_record = sqlx::query_as!(
      UserRecord,
      r#"SELECT * FROM "users" WHERE "username" = $1 LIMIT 1"#,
      username
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| UserRepositoryError::InternalServerError(e.to_string()))?;

    Ok(user_record.map(|record| record.into()))
  }

  async fn find_by_id(
    &mut self,
    id: crate::shared::vo::UUID4,
  ) -> Result<Option<User>, UserRepositoryError> {
    let user_record = sqlx::query_as!(
      UserRecord,
      r#"SELECT * FROM "users" WHERE "id" = $1 LIMIT 1"#,
      Into::<Uuid>::into(id)
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| UserRepositoryError::InternalServerError(e.to_string()))?;

    Ok(user_record.map(|record| record.into()))
  }
}

#[cfg(test)]
mod tests {
  use sqlx::Executor;

  use super::*;
  use crate::{shared::vo::UUID4, AppState};

  const TEST_EMAIL: &str = "user@test.com";

  #[tokio::test]
  async fn should_create_user() {
    // Load .env file
    dotenvy::dotenv().ok();

    async fn delete_old_data() {
      let app_state = AppState::default().await;

      app_state
        .db
        .execute(
          r#"
          DELETE FROM "users" WHERE "id" = '6f76b734-61f7-4613-bdfc-de5064d9fdb1';
          "#,
        )
        .await
        .ok();
    }

    delete_old_data().await;

    let state = AppState::default().await;

    let mut user_repository = SqlxUserRepository::new(&state);

    let username = String::from("test");
    let password = String::from("test");
    let email = String::from(TEST_EMAIL);
    let display_name = String::from("Test User");

    let user = user_repository
      .create(User {
        id: UUID4::new("6f76b734-61f7-4613-bdfc-de5064d9fdb1").unwrap_or_default(),
        username: username.clone(),
        password: password.clone(),
        email: email.clone(),
        display_name: display_name.clone(),
        ..Default::default()
      })
      .await
      .unwrap();

    delete_old_data().await;

    assert_eq!(user.username, username);
    assert_eq!(user.email, email);
    assert_eq!(user.display_name, display_name);
  }

  #[tokio::test]
  async fn should_find_user_by_username() {
    // Load .env file
    dotenvy::dotenv().ok();

    async fn delete_old_data() {
      let app_state = AppState::default().await;

      app_state
        .db
        .execute(
          r#"
          DELETE FROM "users" WHERE "username" = 'should_find_user_by_username';
          "#,
        )
        .await
        .ok();
    }

    let state = AppState::default().await;

    delete_old_data().await;

    let mut user_repository = SqlxUserRepository::new(&state);

    let display_name = String::from("Test User");

    let user = User {
      id: UUID4::new("4661a178-a2ec-4183-ae5f-aa4572860202").unwrap_or_default(),
      username: String::from("should_find_user_by_username"),
      password: String::from("test"),
      email: String::from(TEST_EMAIL),
      display_name: display_name.clone(),
      ..Default::default()
    };

    user_repository.create(user.clone()).await.unwrap();

    let result = user_repository
      .find_by_username(user.username.clone())
      .await
      .unwrap();

    delete_old_data().await;

    assert!(result.is_some(), "User not found");

    let result = result.unwrap();

    assert_eq!(user.id, result.id);
    assert_eq!(user.username, result.username);
    assert_eq!(user.email, result.email);
    assert_eq!(user.display_name, result.display_name);
  }

  #[tokio::test]
  async fn should_not_find_user_by_username() {
    // Load .env file
    dotenvy::dotenv().ok();

    async fn delete_old_data() {
      let app_state = AppState::default().await;

      app_state
        .db
        .execute(
          r#"
          DELETE FROM "users" WHERE "username" = 'test_should_not_find_user_by_username';
          "#,
        )
        .await
        .ok();
    }

    let state: AppState = AppState::default().await;

    delete_old_data().await;

    let mut user_repository = SqlxUserRepository::new(&state);

    let username = String::from("test_should_not_find_user_by_username");

    let user = user_repository
      .find_by_username(username.clone())
      .await
      .unwrap();

    delete_old_data().await;

    assert!(user.is_none());
  }
}
