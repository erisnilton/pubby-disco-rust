use chrono::Utc;
use sqlx::Postgres;
use uuid::Uuid;

use crate::{
  domain::activity::{
    Activity, ActivityChange, ActivityRepository, ActivityRepositoryError, ActivityStatus,
  },
  shared::vo::IntoRecord,
};

pub struct SqlxActivityRepository {
  db: sqlx::Pool<Postgres>,
}

struct ActivityRecord {
  id: Uuid,
  status: String,
  user_id: Uuid,
  curator_id: Option<Uuid>,
  changes: Option<serde_json::Value>,
  entity_name: String,
  entity_id: Option<Uuid>,
  change_type: String,
  revision_date: Option<chrono::DateTime<Utc>>,
  created_at: Option<chrono::DateTime<Utc>>,
  updated_at: Option<chrono::DateTime<Utc>>,
}

impl Into<ActivityRecord> for &Activity {
  fn into(self) -> ActivityRecord {
    ActivityRecord {
      id: Uuid::parse_str(&self.id.0).unwrap(),
      status: match &self.status {
        ActivityStatus::Draft => String::from("Draft"),
        ActivityStatus::Approved => String::from("Approved"),
        ActivityStatus::Pending => String::from("Pending"),
        ActivityStatus::Rejected => String::from("Rejected"),
      },
      user_id: Uuid::parse_str(&self.user.id.0).unwrap(),
      curator_id: self
        .curator
        .as_ref()
        .and_then(|user| Uuid::parse_str(&user.id.0).ok()),
      changes: match &self.change {
        ActivityChange::Create(entity) => Some(IntoRecord::into(entity)),
        ActivityChange::Update { new_value, .. } => Some(IntoRecord::into(new_value)),
        ActivityChange::Delete(..) => None,
      },
      change_type: match self.change {
        ActivityChange::Create(..) => "Create",
        ActivityChange::Delete(..) => "Delete",
        ActivityChange::Update { .. } => "Update",
      }
      .to_string(),
      created_at: Some(self.created_at),
      updated_at: Some(self.updated_at),
      entity_id: match &self.change {
        ActivityChange::Create(..) => None,
        ActivityChange::Delete(entity) => Some(Uuid::parse_str(&entity.id()).unwrap()),
        ActivityChange::Update { entity, .. } => Some(Uuid::parse_str(&entity.id()).unwrap()),
      },
      entity_name: match &self.change {
        ActivityChange::Create(entity) => entity,
        ActivityChange::Delete(entity) => entity,
        ActivityChange::Update { entity, .. } => entity,
      }
      .name(),
      revision_date: self.revision_date,
    }
  }
}

impl SqlxActivityRepository {
  pub fn new(db: sqlx::Pool<Postgres>) -> Self {
    Self { db }
  }
}

impl ActivityRepository for SqlxActivityRepository {
  async fn create(
    &mut self,
    activity: &crate::domain::activity::Activity,
  ) -> Result<Activity, ActivityRepositoryError> {
    let row: ActivityRecord = activity.into();

    sqlx::query!(
      r#"INSERT INTO "activity" (
        "status",
        "user_id",
        "changes",
        "change_type",
        "entity_name",
        "entity_id"
      ) VALUES ( $1::activity_status, $2, $3, $4::activity_change_type, $5, $6)"#,
      row.status as _,
      row.user_id,
      row.changes,
      row.change_type as _,
      row.entity_name,
      row.entity_id
    )
    .execute(&self.db)
    .await
    .map_err(|err| ActivityRepositoryError::InternalServerError(err.to_string()))?;

    Ok(activity.clone())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::{
    domain::{genre::Genre, user::User},
    infra::sqlx::activity_repository::SqlxActivityRepository,
    shared::vo::{CollaborativeEntity, UUID4},
    AppState,
  };

  #[tokio::test]
  async fn test_criar_uma_activity() {
    // Load .env file
    dotenvy::dotenv().ok();

    let activity = Activity {
      user: User {
        id: UUID4::new("e4442384-61ea-440d-be63-cb2642e58007").unwrap_or_default(),
        ..Default::default()
      },
      change: ActivityChange::Create(CollaborativeEntity::Genre(Genre {
        name: String::from("Forró"),
        ..Default::default()
      })),
      ..Default::default()
    };
    let app_state = AppState::default().await;

    let mut activity_repository = SqlxActivityRepository::new(app_state.db.clone());

    let result = activity_repository
      .create(&activity)
      .await
      .expect("falha ao cadastrar atividade");

    assert!(
      activity.id.0 == result.id.0,
      "A atividade retornada não foi a mesma que foi cadastrada"
    );
  }
}
