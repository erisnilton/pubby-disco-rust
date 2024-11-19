#[derive(Debug, Clone, validator::Validate)]
pub struct Input {
  pub activity_id: crate::shared::vo::UUID4,

  #[validate(length(min = 10, max = 255))]
  pub reason: String,
  pub user: crate::domain::user::User,
}

#[derive(Debug)]
pub enum Error {
  UserIsNotACurator,
  ActivityNotFound,
  ActivityRepositoryError(crate::domain::activity::repository::Error),
  ActivityError(crate::domain::activity::Error),
}

pub async fn execute(
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  if !input.user.is_curator {
    return Err(Error::UserIsNotACurator);
  }
  let activity = activity_repository
    .find_by_id(&input.activity_id)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  if let Some(activity) = activity {
    let activity = activity
      .set_curator_status(
        crate::domain::activity::ActivityStatus::Rejected(input.reason),
        &input.user.id,
      )
      .map_err(Error::ActivityError)?;

    activity_repository
      .update(&activity)
      .await
      .map_err(Error::ActivityRepositoryError)?;

    return Ok(activity);
  }

  Err(Error::ActivityNotFound)
}
