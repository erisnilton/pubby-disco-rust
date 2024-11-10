use crate::{
  domain::{
    activity::{dto::CreateActivityDto, Activity, ActivityRepository, ActivityRepositoryError},
    album::{self, AlbumRepository},
    artists::repository::ArtistRepository,
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
  artist_repository: &mut impl ArtistRepository,
  album_repository: &mut impl AlbumRepository,

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
        crate::shared::vo::CollaborativeEntityId::Artist(artist_id) => {
          crate::shared::vo::CollaborativeEntity::Artist(
            match artist_repository.find_by_id(&artist_id).await {
              Ok(Some(artist)) => artist,
              Ok(None) => return Err(ActivityRepositoryError::EntityNotFound),
              Err(e) => {
                log::error!("Failed to find artist: {:?}", e);
                return Err(ActivityRepositoryError::InternalServerError(
                  "Failed to find artist".to_string(),
                ));
              }
            },
          )
        }
        crate::shared::vo::CollaborativeEntityId::Album(album_id) => {
          crate::shared::vo::CollaborativeEntity::Album(
            match album_repository.find_by_id(&album_id).await {
              Ok(Some(album)) => album,
              Ok(None) => return Err(ActivityRepositoryError::EntityNotFound),
              Err(e) => {
                log::error!("Failed to find album: {:?}", e);
                return Err(ActivityRepositoryError::InternalServerError(
                  "Failed to find album".to_string(),
                ));
              }
            },
          )
        }
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
    infra::actix::artist,
    AppState,
  };

  use super::{create_activity, CreateActivityInput};

  use crate::infra::in_memory::InMemoryActivityRepository as ActivityRepository;
  use crate::infra::in_memory::InMemoryAlbumRepository as AlbumRepository;
  use crate::infra::in_memory::InMemoryArtistRepository as ArtistRepository;
  use crate::infra::in_memory::InMemoryGenreRepository as GenreRepository;

  #[tokio::test]
  async fn test_criar_uma_activity() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;
    let mut activity_repository = ActivityRepository::new(&app_state);
    let mut genre_repository = GenreRepository::new(&app_state);
    let mut artist_repository = ArtistRepository::new(&app_state);
    let mut album_repository = AlbumRepository::new(&app_state);

    create_activity(
      &mut activity_repository,
      &mut genre_repository,
      &mut artist_repository,
      &mut album_repository,
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
