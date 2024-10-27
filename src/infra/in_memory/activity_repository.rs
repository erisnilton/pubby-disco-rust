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
}
