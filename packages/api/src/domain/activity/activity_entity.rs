use crate::shared::{self, util::naive_now};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Activity {
  pub id: crate::shared::vo::UUID4,

  pub status: ActivityStatus,
  pub user_id: crate::shared::vo::UUID4,
  pub curator_id: Option<crate::shared::vo::UUID4>,
  pub revision_date: Option<chrono::NaiveDateTime>,
  pub contribuition: shared::vo::Contribution,

  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl Activity {
  pub fn set_curator_status(
    mut self,
    status: ActivityStatus,
    curator_id: &crate::shared::vo::UUID4,
  ) -> Result<Activity, Error> {
    if self.status != ActivityStatus::Pending {
      return Err(Error::ActivityIsNotPending);
    }

    self.status = status;
    self.curator_id = Some(curator_id.clone());
    self.revision_date = Some(naive_now());

    Ok(self)
  }
}

impl Default for Activity {
  fn default() -> Self {
    let now = naive_now();

    Self {
      id: crate::shared::vo::UUID4::default(),
      user_id: crate::shared::vo::UUID4::default(),
      status: ActivityStatus::Pending,
      curator_id: None,
      contribuition: crate::shared::vo::Contribution::default(),
      revision_date: None,
      created_at: now,
      updated_at: now,
    }
  }
}
