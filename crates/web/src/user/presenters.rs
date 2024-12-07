use domain::user::entity::User;
use sqlx::types::chrono;

#[derive(Debug, serde::Serialize)]
pub struct PublicUserPresenter {
  pub id: String,
  pub username: String,
  pub is_curator: bool,
  pub created_at: chrono::NaiveDateTime,
}

impl From<User> for PublicUserPresenter {
  fn from(user: User) -> Self {
    PublicUserPresenter {
      id: user.id().to_string(),
      username: user.username().clone(),
      is_curator: *user.is_curator(),
      created_at: *user.created_at(),
    }
  }
}
