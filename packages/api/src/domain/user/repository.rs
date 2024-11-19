use std::future::Future;

use crate::shared::vo::UUID4;

use super::User;

#[derive(Debug)]
pub enum UserRepositoryError {
  InternalServerError(String),
  Conflict(String),
}

pub trait UserRepository {
  fn create(&mut self, user: User) -> impl Future<Output = Result<User, UserRepositoryError>>;

  fn find_by_username(
    &mut self,
    username: impl Into<String>,
  ) -> impl Future<Output = Result<Option<User>, UserRepositoryError>>;

  fn find_by_id(
    &mut self,
    id: UUID4,
  ) -> impl Future<Output = Result<Option<User>, UserRepositoryError>>;
}
