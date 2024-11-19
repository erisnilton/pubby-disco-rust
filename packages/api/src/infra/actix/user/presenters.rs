use crate::domain::user::User;

#[derive(Debug, serde::Serialize)]
pub struct PublicUserPresenter {
  pub id: String,
  pub username: String,
  pub is_curator: bool,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::domain::user::User> for PublicUserPresenter {
  fn from(user: crate::domain::user::User) -> Self {
    PublicUserPresenter {
      id: user.id().to_string(),
      username: user.username().clone(),
      is_curator: *user.is_curator(),
      created_at: (*user.created_at()).and_utc(),
    }
  }
}

impl From<crate::shared::paged::Paged<User>> for crate::shared::paged::Paged<PublicUserPresenter> {
  fn from(value: crate::shared::paged::Paged<User>) -> Self {
    Self {
      items: value.items.into_iter().map(|item| item.into()).collect(),
      page: value.page,
      total_items: value.total_items,
      total_pages: value.total_pages,
    }
  }
}
