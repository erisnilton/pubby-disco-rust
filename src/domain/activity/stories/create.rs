use crate::{
  domain::{
    activity::{dto::CreateActivityDto, Activity, ActivityRepository, ActivityRepositoryError},
    genre::GenreRepository,
    user::User,
  },
  shared::vo::{CollaborativeEntity, GetChanges},
};

#[derive(Debug)]
pub struct CreateActivityInput {
  pub user: User,
  pub data: CreateActivityDto,
}

pub async fn create_activity(
  activity_repository: &mut impl ActivityRepository,
  genre_repository: &mut impl GenreRepository,
  input: CreateActivityInput,
) -> Result<Activity, ActivityRepositoryError> {
  let activity = match input.data {
    CreateActivityDto::Create(entity) => Activity::create(input.user, entity),
    CreateActivityDto::Update { entity_id, changes } => {
      let entity: CollaborativeEntity = match entity_id {
        crate::shared::vo::CollaborativeEntityId::Genre(genre_id) => {
          crate::shared::vo::CollaborativeEntity::Genre(
            genre_repository.find_by_id(&genre_id).await.unwrap(),
          )
        }
        _ => unreachable!(),
      };

      let (old_value, new_value) = entity.get_changes(changes);

      Activity::update(input.user, entity, old_value, new_value)
    }
  };
  let activity = activity_repository.create(&activity).await?;

  Ok(activity)
}

#[cfg(test)]
pub mod tests {
  use crate::{
    domain::{
      activity::dto::{CreateActivityDto, CreateActivityEntityDto},
      genre::{dto::CreateGenreDto, Genre},
      user::User,
    },
    infra::sqlx::{SqlxActivityRepository, SqlxGenreRepository},
    shared::vo::{CollaborativeEntity, UUID4},
    AppState,
  };

  use super::{create_activity, CreateActivityInput};

  #[tokio::test]
  async fn test_criar_uma_activity() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;
    let mut activity_repository = SqlxActivityRepository::new(app_state.db.clone());
    let mut genre_repository = SqlxGenreRepository::new(app_state.db.clone());

    create_activity(
      &mut activity_repository,
      &mut genre_repository,
      CreateActivityInput {
        user: User {
          id: UUID4::new("e4442384-61ea-440d-be63-cb2642e58007").unwrap_or_default(),
          ..Default::default()
        },
        data: CreateActivityDto::Create(CreateActivityEntityDto::Genre(CreateGenreDto {
          name: String::from("Forró"),
          parent_id: None,
        })),
      },
    )
    .await
    .expect("falha ao cadastrar atividade");
  }
}
