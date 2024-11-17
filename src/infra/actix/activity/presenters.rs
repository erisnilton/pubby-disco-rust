use crate::shared::vo::{Contribution, UUID4};

impl From<crate::domain::activity::ActivityStatus> for ActivityStatus {
  fn from(value: crate::domain::activity::ActivityStatus) -> Self {
    match value {
      crate::domain::activity::ActivityStatus::Pending => Self::Pending,
      crate::domain::activity::ActivityStatus::Approved => Self::Approved,
      crate::domain::activity::ActivityStatus::Rejected(reason) => Self::Rejected(reason),
      crate::domain::activity::ActivityStatus::Draft => Self::Draft,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub enum ActivityStatus {
  Pending,
  Approved,
  Rejected(String),
  Draft,
}

#[derive(Debug, serde::Serialize)]
pub struct PublicActivityPresenter {
  pub id: UUID4,
  pub status: ActivityStatus,
  pub user_id: UUID4,
  pub curator_id: Option<UUID4>,
  pub revision_date: Option<chrono::NaiveDate>,
  pub contribuition: Contribution,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl From<crate::domain::activity::Activity> for PublicActivityPresenter {
  fn from(value: crate::domain::activity::Activity) -> Self {
    PublicActivityPresenter {
      id: value.id,

      status: value.status.into(),
      user_id: value.user_id,
      curator_id: value.curator_id,
      revision_date: value.revision_date.map(|date| date.into()),
      contribuition: value.contribuition.into(),

      created_at: value.created_at.into(),
      updated_at: value.updated_at.into(),
    }
  }
}

// Paged presenter
impl From<crate::shared::paged::Paged<crate::domain::activity::Activity>>
  for crate::shared::paged::Paged<PublicActivityPresenter>
{
  fn from(value: crate::shared::paged::Paged<crate::domain::activity::Activity>) -> Self {
    Self {
      items: value.items.into_iter().map(|item| item.into()).collect(),
      page: value.page,
      total_items: value.total_items,
      total_pages: value.total_pages,
    }
  }
}
