use std::future::Future;

use crate::shared::vo::UUID4;

use super::Activity;

#[derive(Debug)]
pub enum ActivityRepositoryError {
  InternalServerError(String),
  EntityNotFound,
}

pub trait ActivityRepository {
  fn create(
    &mut self,
    activity: &Activity,
  ) -> impl Future<Output = Result<Activity, ActivityRepositoryError>>;

  fn find_by_id(
    &self,
    id: &UUID4,
  ) -> impl Future<Output = Result<Option<Activity>, ActivityRepositoryError>>;

  fn update(
    &mut self,
    activity: &Activity,
  ) -> impl Future<Output = Result<Activity, ActivityRepositoryError>>;
}
