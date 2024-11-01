use chrono::Utc;

use crate::{
  domain::{genre::Genre, user::User},
  shared::vo::{CollaborativeEntity, Slug, UpdateCollaborativeEntityDto, UUID4},
};

use super::dto::CreateActivityEntityDto;

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityStatus {
  Draft,
  Pending,
  Approved,
  Rejected(String),
}

impl From<ActivityStatus> for String {
  fn from(val: ActivityStatus) -> Self {
    match val {
      ActivityStatus::Draft => String::from("Draft"),
      ActivityStatus::Approved => String::from("Approved"),
      ActivityStatus::Pending => String::from("Pending"),
      ActivityStatus::Rejected(reason) => format!("Rejected: {}", reason),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ActivityChange {
  Create(CollaborativeEntity),
  Update {
    entity: CollaborativeEntity,
    old_value: UpdateCollaborativeEntityDto,
    new_value: UpdateCollaborativeEntityDto,
  },
  Delete(CollaborativeEntity),
}

impl ActivityChange {
  pub fn change_name(&self) -> String {
    match self {
      ActivityChange::Create(..) => String::from("Create"),
      ActivityChange::Update { .. } => String::from("Update"),
      ActivityChange::Delete(..) => String::from("Delete"),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ActivityError {
  ActivityIsNotPending,
}

#[derive(Debug, Clone)]
pub struct Activity {
  pub id: UUID4,
  pub status: ActivityStatus,
  pub user: User,
  pub curator: Option<User>,
  pub revision_date: Option<chrono::DateTime<Utc>>,
  pub change: ActivityChange,

  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl Activity {
  pub fn create(user: User, entity: CreateActivityEntityDto) -> Self {
    Self {
      user,
      change: ActivityChange::Create(match entity {
        CreateActivityEntityDto::Genre(data) => CollaborativeEntity::Genre(Genre {
          name: data.name.clone(),
          slug: Slug::generate(&data.name),
          ..Default::default()
        }),
      }),
      ..Default::default()
    }
  }

  pub fn update(
    user: User,
    entity: CollaborativeEntity,
    old_value: UpdateCollaborativeEntityDto,
    new_value: UpdateCollaborativeEntityDto,
  ) -> Self {
    Self {
      user,
      change: ActivityChange::Update {
        entity,
        old_value,
        new_value,
      },
      ..Default::default()
    }
  }

  pub fn set_curator_status(
    mut self,
    status: ActivityStatus,
    curator: &User,
  ) -> Result<Activity, ActivityError> {
    if self.status != ActivityStatus::Pending {
      return Err(ActivityError::ActivityIsNotPending);
    }

    self.status = status;
    self.curator = Some(curator.clone());
    self.revision_date = Some(chrono::Utc::now());

    Ok(self)
  }
}

impl Default for Activity {
  fn default() -> Self {
    Self {
      id: UUID4::default(),
      user: User::default(),
      status: ActivityStatus::Pending,
      curator: None,
      change: ActivityChange::Create(CollaborativeEntity::default()),
      revision_date: None,
      created_at: chrono::Utc::now(),
      updated_at: chrono::Utc::now(),
    }
  }
}
