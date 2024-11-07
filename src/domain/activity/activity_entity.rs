use chrono::Utc;

use crate::{
  domain::{artists::Artist, genre::Genre, user::User},
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

  pub fn entity_name(&self) -> String {
    match self {
      ActivityChange::Create(entity) => entity.name(),
      ActivityChange::Update { entity, .. } => entity.name(),
      ActivityChange::Delete(entity) => entity.name(),
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
        CreateActivityEntityDto::Artist(data) => CollaborativeEntity::Artist(Artist {
          name: data.name.clone(),
          slug: Slug::new(&data.slug).unwrap_or_default(),
          country: data.country.clone(),
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::domain::activity::{ActivityRepository, ActivityStatus};

  #[tokio::test]
  async fn test_fail_when_activity_status_is_not_pending() {
    let user = User {
      username: "user".to_string(),
      password: "password".to_string(),
      email: "user@teste.com".to_string(),
      ..Default::default()
    };
    let curator = User {
      username: "user_curator".to_string(),
      password: "password".to_string(),
      is_curator: true,
      email: "currator@teste.com".to_string(),
      ..Default::default()
    };
    for status in [
      ActivityStatus::Approved,
      ActivityStatus::Draft,
      ActivityStatus::Rejected(String::from("teste")),
    ] {
      let activity = Activity {
        user: user.clone(),
        curator: None,
        status: status.clone(),

        change: crate::domain::activity::ActivityChange::Create(
          crate::shared::vo::CollaborativeEntity::Genre(crate::domain::genre::Genre {
            name: "genre".to_string(),
            slug: crate::shared::vo::Slug::generate("genre"),
            ..Default::default()
          }),
        ),
        ..Default::default()
      };
      let result = activity.set_curator_status(status.clone(), &curator);
      assert!(
        result.is_err(),
        "activity status is {:?} and should return error",
        status
      );
    }
  }

  #[tokio::test]
  // Deve aprovar a atividade se estiver pendente
  async fn test_approve_activity() {
    let user = User {
      username: "user".to_string(),
      password: "password".to_string(),
      email: "user@teste.com".to_string(),
      ..Default::default()
    };

    let curator = User {
      username: "user_curator".to_string(),
      password: "password".to_string(),
      is_curator: true,
      email: "currator@teste.com".to_string(),
      ..Default::default()
    };

    let activity = Activity {
      user: user.clone(),
      curator: None,
      status: ActivityStatus::Pending,

      change: crate::domain::activity::ActivityChange::Create(
        crate::shared::vo::CollaborativeEntity::Genre(crate::domain::genre::Genre {
          name: "genre".to_string(),
          slug: crate::shared::vo::Slug::generate("genre"),
          ..Default::default()
        }),
      ),
      ..Default::default()
    };
    let result = activity.set_curator_status(ActivityStatus::Approved, &curator);
    assert!(
      result.is_ok(),
      "activity status is Pending and should return Ok"
    );
  }

  #[tokio::test]
  // Deve rejeitar a atividade se estiver pendente
  async fn test_reject_activity() {
    let user = User {
      username: "user".to_string(),
      password: "password".to_string(),
      email: "user@teste.com".to_string(),
      ..Default::default()
    };

    let curator = User {
      username: "user_curator".to_string(),
      password: "password".to_string(),
      is_curator: true,
      email: "currator@teste.com".to_string(),
      ..Default::default()
    };

    let activity = Activity {
      user: user.clone(),
      curator: None,
      status: ActivityStatus::Pending,

      change: crate::domain::activity::ActivityChange::Create(
        crate::shared::vo::CollaborativeEntity::Genre(crate::domain::genre::Genre {
          name: "genre".to_string(),
          slug: crate::shared::vo::Slug::generate("genre"),
          ..Default::default()
        }),
      ),
      ..Default::default()
    };
    let result =
      activity.set_curator_status(ActivityStatus::Rejected(String::from("error")), &curator);
    assert!(
      result.is_ok(),
      "activity status is Pending and should return Ok"
    );
  }
}
