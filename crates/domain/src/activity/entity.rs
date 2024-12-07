use entity::Entity;

use shared::{self, util::naive_now, vo::UUID4};

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityStatus {
  Draft,
  Pending,
  Approved,
  Rejected(String),
}

#[derive(Debug, Clone)]
pub enum Error {
  ActivityIsNotPending,
}

#[derive(Entity, Debug, Clone, PartialEq)]
pub struct Activity {
  id: UUID4,

  status: ActivityStatus,
  user_id: UUID4,
  curator_id: Option<UUID4>,
  revision_date: Option<chrono::NaiveDateTime>,
  contribution: crate::activity::vo::Contribution,

  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Activity {
  pub fn set_curator_status(
    mut self,
    status: ActivityStatus,
    curator_id: &UUID4,
  ) -> Result<Activity, Error> {
    if self.status != ActivityStatus::Pending {
      return Err(Error::ActivityIsNotPending);
    }

    self.status = status;
    self.curator_id = Some(curator_id.clone());
    self.revision_date = Some(naive_now());
    self.updated_at = naive_now();

    Ok(self)
  }
}

impl Default for Activity {
  fn default() -> Self {
    let now = naive_now();

    Self {
      id: UUID4::default(),
      user_id: UUID4::default(),
      status: ActivityStatus::Pending,
      curator_id: None,
      contribution: crate::activity::vo::Contribution::default(),
      revision_date: None,
      created_at: now,
      updated_at: now,
    }
  }
}
