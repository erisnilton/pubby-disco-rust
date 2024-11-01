use std::collections::HashMap;

use crate::{
  domain::activity::{Activity, ActivityRepository},
  AppState,
};

#[derive(Debug, Default)]
pub struct InMemoryActivityRepository {
  pub activities: HashMap<String, Activity>,
}

impl InMemoryActivityRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      activities: HashMap::new(),
    }
  }
}

impl ActivityRepository for InMemoryActivityRepository {
  async fn create(
    &mut self,
    input: &Activity,
  ) -> Result<Activity, crate::domain::activity::ActivityRepositoryError> {
    self.activities.insert(input.id.0.clone(), input.clone());
    Ok(input.clone())
  }

  async fn find_by_id(
    &self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<Activity>, crate::domain::activity::ActivityRepositoryError> {
    Ok(self.activities.get(&id.0).cloned())
  }

  async fn update(
    &mut self,
    activity: &Activity,
  ) -> Result<Activity, crate::domain::activity::ActivityRepositoryError> {
    self
      .activities
      .insert(activity.id.0.clone(), activity.clone());
    Ok(activity.clone())
  }
}
