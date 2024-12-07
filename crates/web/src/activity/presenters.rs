use domain::activity::{
  entity::{Activity, ActivityStatus},
  vo::Contribution,
};
use shared::vo::UUID4;

impl From<ActivityStatus> for ActivityStatusDTO {
  fn from(value: ActivityStatus) -> Self {
    match value {
      ActivityStatus::Pending => Self::Pending,
      ActivityStatus::Approved => Self::Approved,
      ActivityStatus::Rejected(reason) => Self::Rejected(reason),
      ActivityStatus::Draft => Self::Draft,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub enum ActivityStatusDTO {
  Pending,
  Approved,
  Rejected(String),
  Draft,
}

#[derive(Debug, serde::Serialize)]
pub struct PublicActivityPresenter {
  pub id: UUID4,
  pub status: ActivityStatusDTO,
  pub user_id: UUID4,
  pub curator_id: Option<UUID4>,
  pub revision_date: Option<chrono::NaiveDate>,
  pub contribution: Contribution,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl From<Activity> for PublicActivityPresenter {
  fn from(value: Activity) -> Self {
    PublicActivityPresenter {
      id: value.id().clone(),

      status: (value.status().clone()).into(),
      user_id: value.user_id().clone(),
      curator_id: value.curator_id().clone(),
      revision_date: value.revision_date().map(|date| date.into()),
      contribution: value.contribution().clone(),

      created_at: *value.created_at(),
      updated_at: *value.updated_at(),
    }
  }
}
