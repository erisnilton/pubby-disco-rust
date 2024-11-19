use crate::*;

#[derive(Debug, Default)]
pub struct InMemoryActivityRepository {
  pub activities: HashMap<String, domain::activity::Activity>,
}

impl InMemoryActivityRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      activities: HashMap::new(),
    }
  }
}

impl domain::activity::repository::ActivityRepository for InMemoryActivityRepository {
  async fn create(
    &mut self,
    input: &domain::activity::Activity,
  ) -> Result<domain::activity::Activity, crate::domain::activity::repository::Error> {
    self
      .activities
      .insert(input.id().to_string(), input.clone());
    Ok(input.clone())
  }

  async fn find_by_id(
    &self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<domain::activity::Activity>, crate::domain::activity::repository::Error> {
    Ok(self.activities.get(&id.to_string()).cloned())
  }

  async fn update(
    &mut self,
    activity: &domain::activity::Activity,
  ) -> Result<domain::activity::Activity, crate::domain::activity::repository::Error> {
    self
      .activities
      .insert(activity.id().to_string(), activity.clone());
    Ok(activity.clone())
  }
}
