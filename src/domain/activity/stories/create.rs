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
            match genre_repository.find_by_id(&genre_id).await {
              Ok(Some(genre)) => genre,
              Ok(None) => return Err(ActivityRepositoryError::EntityNotFound),
              Err(e) => {
                log::error!("Failed to find genre: {:?}", e);
                return Err(ActivityRepositoryError::InternalServerError(
                  "Failed to find genre".to_string(),
                ));
              }
            },
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
      genre::dto::CreateGenreDto,
      user::User,
    },
    AppState,
  };

  use super::{create_activity, CreateActivityInput};

  use crate::infra::in_memory::InMemoryActivityRepository as ActivityRepository;
  use crate::infra::in_memory::InMemoryGenreRepository as GenreRepository;

  #[tokio::test]
  async fn test_criar_uma_activity() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;
    let mut activity_repository = ActivityRepository::new(&app_state);
    let mut genre_repository = GenreRepository::new(&app_state);

    create_activity(
      &mut activity_repository,
      &mut genre_repository,
      CreateActivityInput {
        user: User {
          id: crate::shared::vo::UUID4::new("e4442384-61ea-440d-be63-cb2642e58007")
            .unwrap_or_default(),
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
