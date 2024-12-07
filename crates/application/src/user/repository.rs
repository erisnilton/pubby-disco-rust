use std::future::Future;

use shared::vo::UUID4;

use domain::user::entity::User;

#[derive(Debug)]
pub enum Error {
  InternalServerError(String),
  Conflict(String),
}

pub trait UserRepository {
  fn create(&mut self, user: &User) -> impl Future<Output = Result<User, Error>>;

  fn find_by_username(
    &mut self,
    username: impl Into<String>,
  ) -> impl Future<Output = Result<Option<User>, Error>>;

  fn find_by_id(&mut self, id: UUID4) -> impl Future<Output = Result<Option<User>, Error>>;
}
