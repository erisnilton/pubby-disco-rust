use domain::{
  activity::entity::{Activity, ActivityStatus},
  user::entity::User,
};
use shared::vo::UUID4;

#[derive(Debug, Clone, validator::Validate)]
pub struct Input {
  pub activity_id: UUID4,

  #[validate(length(min = 10, max = 255))]
  pub reason: String,
  pub user: User,
}

#[derive(Debug)]
pub enum Error {
  UserIsNotACurator,
  ActivityNotFound,
  ActivityRepositoryError(crate::activity::repository::Error),
  ActivityError(domain::activity::entity::Error),
}

pub async fn execute(
  activity_repository: &mut impl crate::activity::repository::ActivityRepository,
  input: Input,
) -> Result<Activity, Error> {
  if !input.user.is_curator() {
    return Err(Error::UserIsNotACurator);
  }
  let activity = activity_repository
    .find_by_id(&input.activity_id)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  if let Some(activity) = activity {
    let activity = activity
      .set_curator_status(ActivityStatus::Rejected(input.reason), input.user.id())
      .map_err(Error::ActivityError)?;

    activity_repository
      .update(&activity)
      .await
      .map_err(Error::ActivityRepositoryError)?;

    return Ok(activity);
  }

  Err(Error::ActivityNotFound)
}
