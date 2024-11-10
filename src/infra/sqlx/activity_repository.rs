use sqlx::Postgres;
use uuid::Uuid;

use crate::{
  domain::{
    activity::{
      Activity, ActivityChange, ActivityRepository, ActivityRepositoryError, ActivityStatus,
    },
    genre::Genre,
    user::User,
  },
  shared::vo::{IntoRecord, UUID4},
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
  reason: Option<String>,
  revision_date: Option<chrono::NaiveDateTime>,
  created_at: Option<chrono::NaiveDateTime>,
  updated_at: Option<chrono::NaiveDateTime>,
}

impl Into<ActivityRecord> for &Activity {
  fn into(self) -> ActivityRecord {
    ActivityRecord {
      id: Uuid::parse_str(&self.id.0).unwrap(),
      status: match &self.status {
        ActivityStatus::Draft => String::from("Draft"),
        ActivityStatus::Approved => String::from("Approved"),
        ActivityStatus::Pending => String::from("Pending"),
        ActivityStatus::Rejected(_) => String::from("Rejected"),
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
        ActivityChange::Create(..) => String::from("Create"),
        ActivityChange::Update { .. } => String::from("Update"),
        ActivityChange::Delete(..) => String::from("Delete"),
      },
      created_at: Some(self.created_at.naive_utc()),
      updated_at: Some(self.updated_at.naive_utc()),
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
      .name()
      .into(),
      revision_date: self.revision_date.map(|date| date.naive_utc()),
      reason: match &self.status {
        ActivityStatus::Rejected(reason) => Some(reason.clone()),
        _ => None,
      },
    }
  }
}

impl From<ActivityRecord> for Activity {
  fn from(value: ActivityRecord) -> Self {
    Activity {
      id: UUID4::new(value.id.to_string()).unwrap_or_default(),
      revision_date: value.revision_date.map(|date| date.and_utc()),
      curator: None,
      user: User::default(),
      status: match value.status.as_str() {
        "Draft" => ActivityStatus::Draft,
        "Approved" => ActivityStatus::Approved,
        "Pending" => ActivityStatus::Pending,
        "Rejected" => ActivityStatus::Rejected(value.reason.unwrap_or_default()),
        _ => ActivityStatus::Draft,
      },
      change: match value.change_type.as_str() {
        "Create" => ActivityChange::Create(match value.entity_name.as_str() {
          "Genre" => {
            crate::shared::vo::CollaborativeEntity::Genre(value.changes.unwrap_or_default().into())
          }
          "Artist" => {
            crate::shared::vo::CollaborativeEntity::Artist(value.changes.unwrap_or_default().into())
          }
          "Album" => {
            crate::shared::vo::CollaborativeEntity::Album(value.changes.unwrap_or_default().into())
          }
          value => panic!("Unexpected entity name: {}", value),
        }),
        "Update" => ActivityChange::Update {
          entity: match value.entity_name.as_str() {
            "Genre" => crate::shared::vo::CollaborativeEntity::Genre(Genre::default()),
            "Artist" => crate::shared::vo::CollaborativeEntity::Artist(Default::default()),
            "Album" => crate::shared::vo::CollaborativeEntity::Album(Default::default()),
            value => panic!("Unexpected entity name: {}", value),
          },
          new_value: match value.entity_name.as_str() {
            "Genre" => crate::shared::vo::UpdateCollaborativeEntity::Genre(
              value.changes.clone().unwrap_or_default().into(),
            ),
            "Artist" => crate::shared::vo::UpdateCollaborativeEntity::Artist(
              value.changes.clone().unwrap_or_default().into(),
            ),
            "Album" => crate::shared::vo::UpdateCollaborativeEntity::Album(
              value.changes.clone().unwrap_or_default().into(),
            ),
            value => panic!("Unexpected entity name: {}", value),
          },
          old_value: match value.entity_name.as_str() {
            "Genre" => crate::shared::vo::UpdateCollaborativeEntity::Genre(
              value.changes.clone().unwrap_or_default().into(),
            ),
            "Artist" => crate::shared::vo::UpdateCollaborativeEntity::Artist(
              value.changes.clone().unwrap_or_default().into(),
            ),
            "Album" => crate::shared::vo::UpdateCollaborativeEntity::Album(
              value.changes.clone().unwrap_or_default().into(),
            ),
            value => panic!("Unexpected entity name: {}", value),
          },
        },
        "Delete" => ActivityChange::Delete(match value.entity_name.as_str() {
          "Genre" => {
            crate::shared::vo::CollaborativeEntity::Genre(value.changes.unwrap_or_default().into())
          }
          "Artist" => {
            crate::shared::vo::CollaborativeEntity::Artist(value.changes.unwrap_or_default().into())
          }
          "Album" => {
            crate::shared::vo::CollaborativeEntity::Album(value.changes.unwrap_or_default().into())
          }
          value => panic!("Unexpected entity name: {}", value),
        }),
        v => panic!("Unexpected change type: {}", v),
      },
      created_at: value
        .created_at
        .map(|date| date.and_utc())
        .unwrap_or_default(),
      updated_at: value
        .updated_at
        .map(|date| date.and_utc())
        .unwrap_or_default(),
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

    sqlx::query_as!(
      ActivityRecord,
      r#"INSERT INTO "activity" (
        "id",
        "status",
        "user_id",
        "changes",
        "change_type",
        "entity_name",
        "entity_id"
      ) VALUES ( $1, $2::activity_status, $3, $4, $5::activity_change_type, $6, $7)"#,
      row.id,
      Into::<String>::into(row.status) as _,
      row.user_id,
      row.changes,
      Into::<String>::into(row.change_type) as _,
      row.entity_name,
      row.entity_id
    )
    .execute(&self.db)
    .await
    .map_err(|err| ActivityRepositoryError::InternalServerError(err.to_string()))?;

    Ok(activity.clone())
  }

  async fn find_by_id(
    &self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<Activity>, ActivityRepositoryError> {
    let result = sqlx::query_as!(
      ActivityRecord,
      r#"SELECT
        "id",
        "status" as "status: String",
        "user_id",
        "curator_id",
        "changes",
        "change_type" as "change_type: String",
        "entity_name",
        "entity_id",
        "revision_date",
        "reason",
        "created_at",
        "updated_at"
        FROM "activity"
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| ActivityRepositoryError::InternalServerError(err.to_string()))?;

    Ok(result.map(|row| row.into()))
  }

  async fn update(&mut self, activity: &Activity) -> Result<Activity, ActivityRepositoryError> {
    let row: ActivityRecord = activity.into();

    sqlx::query_as!(
      ActivityRecord,
      r#"UPDATE "activity" SET
            "status" = $2::activity_status,
            "curator_id" = $3,
            "changes" = $4,
            "revision_date" = $5,
            "updated_at" = $6,
            "reason" = $7

          WHERE "id" = $1"#,
      row.id,
      Into::<String>::into(row.status.clone()) as _,
      row.curator_id,
      row.changes,
      row.revision_date,
      row.updated_at,
      match &activity.status {
        ActivityStatus::Rejected(reason) => Some(reason),
        _ => None,
      }
    )
    .execute(&self.db)
    .await
    .map_err(|err| ActivityRepositoryError::InternalServerError(err.to_string()))?;
    Ok(activity.clone())
  }
}

#[cfg(test)]
pub mod tests {
  use sqlx::Executor;

  use super::*;
  use crate::{
    domain::{
      genre::Genre,
      user::{User, UserRepository},
    },
    infra::sqlx::{
      activity_repository::SqlxActivityRepository, user_repository, SqlxUserRepository,
    },
    shared::vo::{CollaborativeEntity, UUID4},
    AppState,
  };

  #[tokio::test]
  async fn test_criar_uma_activity() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;
    let user_id = UUID4::new("e4442384-61ea-440d-be63-cb2642e58007").unwrap_or_default();
    let activity_id = UUID4::new("05188e82-6566-45db-8914-154c587cb951").unwrap_or_default();

    async fn delete_old_data() {
      let app_state = AppState::default().await;

      app_state
        .db
        .execute(
          r#"
          DELETE FROM "activity" WHERE "user_id" = 'e4442384-61ea-440d-be63-cb2642e58007';
          DELETE FROM "users" WHERE "id" = 'e4442384-61ea-440d-be63-cb2642e58007';
          "#,
        )
        .await
        .ok();
    }

    let user = User {
      id: user_id.clone(),
      ..Default::default()
    };

    delete_old_data().await;

    let mut user_repository = SqlxUserRepository::new(&app_state);
    user_repository.create(user.clone()).await.ok();

    let activity = Activity {
      id: activity_id.clone(),
      user: user.clone(),

      change: ActivityChange::Create(CollaborativeEntity::Genre(Genre {
        name: String::from("Forró"),
        ..Default::default()
      })),
      ..Default::default()
    };

    let mut activity_repository = SqlxActivityRepository::new(app_state.db.clone());

    let result = activity_repository
      .create(&activity)
      .await
      .expect("falha ao cadastrar atividade");

    delete_old_data().await;

    assert!(
      activity.id.0 == result.id.0,
      "A atividade retornada não foi a mesma que foi cadastrada"
    );
  }

  #[tokio::test]
  async fn test_buscar_uma_activity() {
    // Load .env file
    dotenvy::dotenv().ok();

    async fn delete_old_data() {
      let app_state = AppState::default().await;

      app_state
        .db
        .execute(
          r#"
          DELETE FROM "activity" WHERE "user_id" = 'b9a2a549-d6c4-4067-8ad3-f79eac1d3393';
          DELETE FROM "users" WHERE "id" = 'b9a2a549-d6c4-4067-8ad3-f79eac1d3393';
          "#,
        )
        .await
        .ok();
    }

    let app_state = AppState::default().await;

    let user_id = UUID4::new("b9a2a549-d6c4-4067-8ad3-f79eac1d3393").unwrap_or_default();
    let activity_id = UUID4::new("89932d3d-9fc8-48be-82a2-7aa6ad1a4036").unwrap_or_default();

    let user = User {
      id: user_id.clone(),
      username: String::from("user"),
      password: String::from("password"),
      email: String::from("test@test.com"),
      ..Default::default()
    };

    let activity = Activity {
      id: activity_id.clone(),
      user: user.clone(),
      change: ActivityChange::Create(CollaborativeEntity::Genre(Genre {
        name: String::from("Forró"),
        ..Default::default()
      })),
      ..Default::default()
    };

    delete_old_data().await;

    let mut user_repository = SqlxUserRepository::new(&app_state);

    user_repository.create(user.clone()).await.unwrap();

    let mut activity_repository = SqlxActivityRepository::new(app_state.db.clone());

    activity_repository.create(&activity).await.unwrap();

    let result = activity_repository
      .find_by_id(&activity_id)
      .await
      .expect("falha ao buscar atividade");

    println!("{:?}", result);

    assert!(result.is_some(), "Atividade não encontrada");
    assert!(
      result.unwrap().id == activity_id,
      "A atividade retornada não foi a mesma que foi cadastrada"
    );
  }
}
