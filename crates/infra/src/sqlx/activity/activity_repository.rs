use ::sqlx;
use application::activity::repository::ActivityRepository;
use domain::activity::entity::{Activity, ActivityStatus};
use shared::vo::UUID4;

pub struct SqlxActivityRepository {
  db: sqlx::PgPool,
}

impl SqlxActivityRepository {
  pub fn new(db: sqlx::PgPool) -> Self {
    Self { db }
  }
}

impl ActivityRepository for SqlxActivityRepository {
  async fn create(
    &mut self,
    activity: &Activity,
  ) -> Result<Activity, application::activity::repository::Error> {
    sqlx::query!(
      r#"INSERT INTO "activity" (
        "id",
        "status",
        "user_id",
        "changes",
        "created_at",
        "updated_at"
      ) VALUES ($1, $2::activity_status, $3, $4, $5, $6)"#,
      Into::<uuid::Uuid>::into(activity.id().clone()),
      match activity.status() {
        ActivityStatus::Draft => "Draft",
        ActivityStatus::Pending => "Pending",
        ActivityStatus::Approved => "Approved",
        ActivityStatus::Rejected(_) => "Rejected",
      } as _,
      Into::<uuid::Uuid>::into(activity.user_id().clone()),
      serde_json::to_value(activity.contribution()).expect("falha ao serializar contribuição"),
      activity.created_at(),
      activity.updated_at()
    )
    .execute(&self.db)
    .await
    .map_err(|err| {
      application::activity::repository::Error::InternalServerError(err.to_string())
    })?;

    Ok(activity.clone())
  }

  async fn find_by_id(
    &self,
    id: &UUID4,
  ) -> Result<Option<Activity>, application::activity::repository::Error> {
    let result = sqlx::query!(
      r#"SELECT
        "id",
        "status" as "status: String",
        "user_id",
        "curator_id",
        "changes",
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
    .map_err(|err| {
      application::activity::repository::Error::InternalServerError(err.to_string())
    })?;

    Ok(result.map(|row| {
      Activity::builder()
        .id(UUID4::new(row.id).unwrap())
        .status(match row.status.as_str() {
          "Draft" => ActivityStatus::Draft,
          "Pending" => ActivityStatus::Pending,
          "Approved" => ActivityStatus::Approved,
          "Rejected" => ActivityStatus::Rejected(row.reason.unwrap_or_default()),
          value => panic!("status {:?} desconhecido", value),
        })
        .user_id(UUID4::new(row.user_id).unwrap())
        .curator_id(row.curator_id.map(|id| UUID4::new(id).unwrap()))
        .revision_date(row.revision_date)
        .contribution(
          serde_json::from_value(row.changes.expect("contribuição não encontrada"))
            .expect("falha ao deserializar contribuição"),
        )
        .created_at(row.created_at)
        .updated_at(row.updated_at)
        .build()
    }))
  }

  async fn update(
    &mut self,
    activity: &Activity,
  ) -> Result<Activity, application::activity::repository::Error> {
    sqlx::query!(
      r#"UPDATE "activity" SET
            "status" = $2::activity_status,
            "curator_id" = $3,
            "changes" = $4,
            "revision_date" = $5,
            "updated_at" = $6,
            "reason" = $7

          WHERE "id" = $1"#,
      Into::<uuid::Uuid>::into(activity.id().clone()),
      match activity.status() {
        ActivityStatus::Draft => "Draft",
        ActivityStatus::Pending => "Pending",
        ActivityStatus::Approved => "Approved",
        ActivityStatus::Rejected(_) => "Rejected",
      } as _,
      activity
        .curator_id()
        .clone()
        .map(|id| Into::<uuid::Uuid>::into(id)),
      serde_json::to_value(activity.contribution()).expect("falha ao serializar contribuição"),
      activity.revision_date().clone(),
      activity.updated_at(),
      match activity.status() {
        ActivityStatus::Rejected(reason) => Some(reason.clone()),
        _ => None,
      }
    )
    .execute(&self.db)
    .await
    .map_err(|err| {
      application::activity::repository::Error::InternalServerError(err.to_string())
    })?;
    Ok(activity.clone())
  }
}

#[cfg(test)]
pub mod tests {
  // use domain::{activity::repository::ActivityRepository, user::UserRepository};
  // use sqlx::Executor;

  // use crate::*;

  // #[tokio::test]
  // async fn test_criar_uma_activity() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const USER_ID: &str = "e4442384-61ea-440d-be63-cb2642e58007";
  //   const ACTIVITY_ID: &str = "05188e82-6566-45db-8914-154c587cb951";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     db.execute(
  //       format!(
  //         r#"
  //         DELETE FROM "activity" WHERE "id" = '{}';
  //         DELETE FROM "users" WHERE "id" = '{}';
  //         "#,
  //         ACTIVITY_ID, USER_ID
  //       )
  //       .as_str(),
  //     )
  //     .await
  //     .expect("falha ao deletar dados antigos");
  //   }

  //   let user = domain::user::User::builder()
  //     .id(UUID4::new(USER_ID).unwrap())
  //     .username(String::from("test_criar_uma_activity_user"))
  //     .password(String::from("password"))
  //     .build();

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut user_repository = infra::sqlx::SqlxUserRepository::new(&app_state);

  //   user_repository
  //     .create(&user)
  //     .await
  //     .expect("falha ao cadastrar usuário");

  //   let activity = Activity::builder()
  //     .id(UUID4::new(ACTIVITY_ID).unwrap())
  //     .user_id(user.id().clone())
  //     .contribution(shared::vo::Contribution::Genre(
  //       domain::genre::contribution::Contribution::Create(
  //         domain::genre::Genre::builder()
  //           .name(String::from("Forró"))
  //           .build(),
  //       ),
  //     ))
  //     .build();

  //   let mut activity_repository = infra::sqlx::SqlxActivityRepository::new(&app_state);

  //   activity_repository
  //     .create(&activity)
  //     .await
  //     .expect("falha ao cadastrar atividade");

  //   let result = activity_repository
  //     .find_by_id(activity.id())
  //     .await
  //     .expect("falha ao buscar atividade")
  //     .expect("atividade não encontrada");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(
  //     activity, result,
  //     "A atividade retornada não foi a mesma que foi cadastrada"
  //   );
  // }

  // #[tokio::test]
  // async fn test_buscar_uma_atividade() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const USER_ID: &str = "b9a2a549-d6c4-4067-8ad3-f79eac1d3393";
  //   const ACTIVITY_ID: &str = "89932d3d-9fc8-48be-82a2-7aa6ad1a4036";

  //   async fn clean(db: &sqlx::PgPool) {
  //     db.execute(
  //       format!(
  //         r#"
  //         DELETE FROM "activity" WHERE "id" = '{}';
  //         DELETE FROM "users" WHERE "id" = '{}';
  //         "#,
  //         ACTIVITY_ID, USER_ID
  //       )
  //       .as_str(),
  //     )
  //     .await
  //     .expect("falha ao deletar dados antigos");
  //   }

  //   let user = domain::user::User::builder()
  //     .id(UUID4::new(USER_ID).unwrap())
  //     .username(String::from("test_buscar_uma_activity_user"))
  //     .password(String::from("password"))
  //     .email(String::from("test@test.com"))
  //     .build();

  //   let activity = Activity::builder()
  //     .id(UUID4::new(ACTIVITY_ID).unwrap())
  //     .user_id(user.id().clone())
  //     .contribution(shared::vo::Contribution::Genre(
  //       domain::genre::contribution::Contribution::Create(
  //         domain::genre::Genre::builder()
  //           .name(String::from("Forró"))
  //           .build(),
  //       ),
  //     ))
  //     .build();

  //   let app_state = AppState::default().await;

  //   clean(&app_state.db).await;

  //   let mut user_repository = infra::sqlx::SqlxUserRepository::new(&app_state);

  //   user_repository
  //     .create(&user)
  //     .await
  //     .expect("falha ao cadastrar usuário");

  //   let mut activity_repository = infra::sqlx::SqlxActivityRepository::new(&app_state);

  //   activity_repository
  //     .create(&activity)
  //     .await
  //     .expect("falha ao cadastrar atividade");

  //   let result = activity_repository
  //     .find_by_id(activity.id())
  //     .await
  //     .expect("falha ao buscar atividade")
  //     .expect("atividade não encontrada");

  //   clean(&app_state.db).await;

  //   assert_eq!(
  //     activity, result,
  //     "A atividade retornada não foi a mesma que foi cadastrada"
  //   );
  // }
}
