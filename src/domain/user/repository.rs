use std::future::Future;

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
}
