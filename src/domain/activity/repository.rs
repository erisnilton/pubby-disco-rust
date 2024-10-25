use std::future::Future;

use super::Activity;

#[derive(Debug)]
pub enum ActivityRepositoryError {
  InternalServerError(String),
}

pub trait ActivityRepository {
  fn create(
    &mut self,
    activity: &Activity,
  ) -> impl Future<Output = Result<Activity, ActivityRepositoryError>>;
}
