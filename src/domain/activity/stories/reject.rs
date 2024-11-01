use crate::{
  domain::{
    activity::{Activity, ActivityRepository},
    user::User,
  },
  shared::vo::UUID4,
};

#[derive(Debug, Clone, validator::Validate)]
pub struct Input {
  pub activity_id: UUID4,

  #[validate(length(min = 10, max = 255))]
  pub reason: String,
  pub user: User,
}

#[derive(Debug)]
pub enum RejectActivityError {
  UserIsNotACurator,
  RepositoryError(crate::domain::activity::ActivityRepositoryError),
  ActivityNotFound,
  ActivityError(crate::domain::activity::ActivityError),
}

pub async fn execute(
  activity_repository: &mut impl ActivityRepository,
  input: Input,
) -> Result<Activity, RejectActivityError> {
  if !input.user.is_curator {
    return Err(RejectActivityError::UserIsNotACurator);
  }
  let activity = activity_repository
    .find_by_id(&input.activity_id)
    .await
    .map_err(RejectActivityError::RepositoryError)?;

  if let Some(activity) = activity {
    let activity = activity
      .set_curator_status(
        crate::domain::activity::ActivityStatus::Rejected(input.reason),
        &input.user,
      )
      .map_err(RejectActivityError::ActivityError)?;
    activity_repository
      .update(&activity)
      .await
      .map_err(RejectActivityError::RepositoryError)?;
    return Ok(activity);
  }

  Err(RejectActivityError::ActivityNotFound)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    domain::{activity::ActivityStatus, user::UserRepository},
    infra::in_memory::{InMemoryActivityRepository, InMemoryUserRepository},
  };

  #[tokio::test]
  async fn test_reject_activity() {
    // Load .env file
    dotenvy::dotenv().ok();
    let app_state = crate::AppState::default().await;
    let mut activity_repository = InMemoryActivityRepository::new(&app_state);
    let mut user_repository = InMemoryUserRepository::new(&app_state);
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
      reason: "reason".to_string(),
      user: curator.clone(),
    };

    let result = execute(&mut activity_repository, input).await.unwrap();
    assert_eq!(result.id, activity.id);
    assert_eq!(
      result.status,
      crate::domain::activity::ActivityStatus::Rejected("reason".to_string())
    );
    assert!(
      result.revision_date.is_some(),
      "revision_date should be Some"
    );
  }

  #[tokio::test]
  async fn test_fail_when_activity_status_is_not_pending() {
    // Load .env file
    dotenvy::dotenv().ok();
    let app_state = crate::AppState::default().await;
    let mut activity_repository = InMemoryActivityRepository::new(&app_state);
    let mut user_repository = InMemoryUserRepository::new(&app_state);
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

    user_repository.create(user.clone()).await.unwrap();
    user_repository.create(curator.clone()).await.unwrap();
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
      activity_repository.create(&activity).await.unwrap();
      let input = Input {
        activity_id: activity.id.clone(),
        reason: "reason".to_string(),
        user: curator.clone(),
      };
      let result = execute(&mut activity_repository, input).await;
      assert!(
        result.is_err(),
        "activity status is {:?} and should return error",
        status
      );
    }
  }
}
