use application::user::repository::UserRepository;
use domain::user::entity::User;
use shared::vo::UUID4;
use uuid::Uuid;

pub struct SqlxUserRepository {
  db: sqlx::PgPool,
}

impl SqlxUserRepository {
  pub fn new(db: sqlx::PgPool) -> Self {
    Self { db }
  }
}

impl UserRepository for SqlxUserRepository {
  async fn create(&mut self, user: &User) -> Result<User, application::user::repository::Error> {
    sqlx::query!(
      r#"
      INSERT INTO "users" ("id", "username", "password", "email", "display_name")
      VALUES ($1, $2, $3, $4, $5)
    "#r,
      uuid::Uuid::from(user.id().clone()),
      user.username(),
      user.password(),
      user.email(),
      user.display_name()
    )
    .execute(&self.db)
    .await
    .map_err(|e| application::user::repository::Error::InternalServerError(e.to_string()))?;

    Ok(user.clone())
  }

  async fn find_by_username(
    &mut self,
    username: impl Into<String>,
  ) -> Result<Option<User>, application::user::repository::Error> {
    let username: String = username.into();

    let user_record = sqlx::query!(
      r#"SELECT * FROM "users" WHERE "username" = $1 LIMIT 1"#,
      username
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|e| application::user::repository::Error::InternalServerError(e.to_string()))?;

    Ok(user_record.map(|record| {
      User::builder()
        .id(record.id.into())
        .username(record.username)
        .password(record.password)
        .email(record.email)
        .display_name(record.display_name)
        .is_curator(record.is_curator)
        .created_at(record.created_at)
        .updated_at(record.updated_at)
        .build()
    }))
  }

  async fn find_by_id(
    &mut self,
    id: UUID4,
  ) -> Result<Option<User>, application::user::repository::Error> {
    let user_record = sqlx::query!(
      r#"SELECT * FROM "users" WHERE "id" = $1 LIMIT 1"#,
      Into::<Uuid>::into(id)
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|e| application::user::repository::Error::InternalServerError(e.to_string()))?;

    Ok(user_record.map(|record| {
      User::builder()
        .id(record.id.into())
        .username(record.username)
        .password(record.password)
        .email(record.email)
        .display_name(record.display_name)
        .is_curator(record.is_curator)
        .created_at(record.created_at)
        .updated_at(record.updated_at)
        .build()
    }))
  }
}

#[cfg(test)]
mod tests {
  // use sqlx::Executor;

  // use super::*;
  // use crate::{shared::vo::UUID4, AppState};

  // const TEST_EMAIL: &str = "user@test.com";

  // #[tokio::test]
  // async fn should_create_user() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   async fn delete_old_data() {
  //     let app_state = AppState::default().await;

  //     app_state
  //       .db
  //       .execute(
  //         r#"
  //         DELETE FROM "users" WHERE "id" = '6f76b734-61f7-4613-bdfc-de5064d9fdb1';
  //         "#,
  //       )
  //       .await
  //       .ok();
  //   }

  //   delete_old_data().await;

  //   let state = AppState::default().await;

  //   let mut user_repository = SqlxUserRepository::new(&state);

  //   let username = String::from("test");
  //   let password = String::from("test");
  //   let email = String::from(TEST_EMAIL);
  //   let display_name = String::from("Test User");

  //   let user = User::builder()
  //     .id(UUID4::new("6f76b734-61f7-4613-bdfc-de5064d9fdb1").unwrap())
  //     .username(username.clone())
  //     .password(password.clone())
  //     .email(email.clone())
  //     .display_name(display_name.clone())
  //     .build();

  //   let user2 = user_repository.create(&user).await.unwrap();

  //   delete_old_data().await;

  //   assert_eq!(
  //     user, user2,
  //     "O usuário retornado não é o mesmo que foi criado"
  //   );
  // }

  // #[tokio::test]
  // async fn should_find_user_by_username() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   async fn delete_old_data() {
  //     let app_state = AppState::default().await;

  //     app_state
  //       .db
  //       .execute(
  //         r#"
  //         DELETE FROM "users" WHERE "username" = 'should_find_user_by_username';
  //         "#,
  //       )
  //       .await
  //       .ok();
  //   }

  //   let state = AppState::default().await;

  //   delete_old_data().await;

  //   let mut user_repository = SqlxUserRepository::new(&state);

  //   let display_name = String::from("Test User");

  //   let user = User::builder()
  //     .id(UUID4::new("4661a178-a2ec-4183-ae5f-aa4572860202").unwrap())
  //     .username(String::from("should_find_user_by_username"))
  //     .password(String::from("test"))
  //     .email(String::from(TEST_EMAIL))
  //     .display_name(display_name.clone())
  //     .build();

  //   user_repository.create(&user).await.unwrap();

  //   let result = user_repository
  //     .find_by_username(user.username())
  //     .await
  //     .expect("Erro ao buscar usuário por username")
  //     .expect("Usuário não encontrado");

  //   delete_old_data().await;

  //   assert_eq!(
  //     user, result,
  //     "Usuário retornado não é o mesmo que foi criado"
  //   );
  // }

  // #[tokio::test]
  // async fn should_not_find_user_by_username() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   async fn delete_old_data() {
  //     let app_state = AppState::default().await;

  //     app_state
  //       .db
  //       .execute(
  //         r#"
  //         DELETE FROM "users" WHERE "username" = 'test_should_not_find_user_by_username';
  //         "#,
  //       )
  //       .await
  //       .ok();
  //   }

  //   let state: AppState = AppState::default().await;

  //   delete_old_data().await;

  //   let mut user_repository = SqlxUserRepository::new(&state);

  //   let username = String::from("test_should_not_find_user_by_username");

  //   let user = user_repository
  //     .find_by_username(username.clone())
  //     .await
  //     .unwrap();

  //   delete_old_data().await;

  //   assert!(user.is_none());
  // }
}
