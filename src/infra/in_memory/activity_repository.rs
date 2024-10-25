use std::collections::HashMap;

use crate::domain::activity::{Activity, ActivityRepository};

#[derive(Debug, Default)]
pub struct InMemoryActivityRepository {
  pub activities: HashMap<String, Activity>,
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
