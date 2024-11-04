use crate::{
  domain::{
    self,
    activity::{error::EntityUpdateError, Activity, ActivityStatus},
    user::User,
  },
  shared::vo::UUID4,
};

#[derive(Debug, Clone)]
pub enum ApproveActivityError {
  UserIsNotACurator,
  RepositoryError(crate::domain::activity::ActivityRepositoryError),
  ActivityNotFound,
  ActivityError(crate::domain::activity::ActivityError),
  EntityUpdateError(crate::domain::activity::error::EntityUpdateError),
  InvalidEntity,
}

#[derive(Debug, Clone)]
pub struct Input {
  pub activity_id: UUID4,
  pub actor: User,
}

pub async fn execute(
  activity_repository: &mut impl crate::domain::activity::ActivityRepository,
  repository_genre: &mut impl crate::domain::genre::GenreRepository,
  input: Input,
) -> Result<Activity, ApproveActivityError> {
  if !input.actor.is_curator {
    return Err(ApproveActivityError::UserIsNotACurator);
  }

  let activity = activity_repository
    .find_by_id(&input.activity_id)
    .await
    .map_err(ApproveActivityError::RepositoryError)?;

  if let Some(mut activity) = activity {
    activity = activity
      .set_curator_status(ActivityStatus::Approved, &input.actor)
      .map_err(ApproveActivityError::ActivityError)?;

    match activity.change.entity_name().as_str() {
      "Genre" => {
        domain::genre::stories::apply_changes::execute(repository_genre, activity.change.clone())
          .await
          .map_err(|err| ApproveActivityError::EntityUpdateError(EntityUpdateError::Genre(err)))?;
      }
      _ => return Err(ApproveActivityError::InvalidEntity),
    }

    return Ok(activity);
  }
  Err(ApproveActivityError::ActivityNotFound)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    domain::{
      activity::{ActivityRepository, ActivityStatus},
      user::UserRepository,
    },
    infra::in_memory::{InMemoryActivityRepository, InMemoryUserRepository},
  };

  #[tokio::test]
  async fn test_approve_activity() {
    // Load .env file
    dotenvy::dotenv().ok();
    let app_state = crate::AppState::default().await;

    let mut activity_repository = InMemoryActivityRepository::new(&app_state);
    let mut user_repository = InMemoryUserRepository::new(&app_state);
    let mut genre_repository = crate::infra::in_memory::InMemoryGenreRepository::new(&app_state);

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

      change: crate::domain::activity::ActivityChange::Create(
        crate::shared::vo::CollaborativeEntity::Genre(crate::domain::genre::Genre {
          name: "genre".to_string(),
          slug: crate::shared::vo::Slug::generate("genre"),
          ..Default::default()
        }),
      ),
      ..Default::default()
    };

    user_repository.create(user.clone()).await.unwrap();
    user_repository.create(curator.clone()).await.unwrap();
    activity_repository.create(&activity).await.unwrap();

    let input = Input {
      activity_id: activity.id.clone(),
      actor: curator.clone(),
    };

    let result = execute(&mut activity_repository, &mut genre_repository, input)
      .await
      .unwrap();
    assert_eq!(result.id, activity.id);
    assert_eq!(result.status, ActivityStatus::Approved);
  }
}
