use domain::{
  activity::entity::{Activity, ActivityStatus},
  source::entity::{Source, SourceType},
};
use shared::vo::UUID4;

use crate::{activity::repository::ActivityRepository, source::repository::SourceRepository};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSourceInput {
  pub source_type: SourceType,
  pub src: String,
  pub media_id: UUID4,
}

impl From<CreateSourceInput> for Source {
  fn from(value: CreateSourceInput) -> Self {
    Self::builder()
      .source_type(value.source_type)
      .src(value.src)
      .media_id(value.media_id)
      .build()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateSourceInput),
  Delete(UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  SourceNotFound,
  SourceRepositoryError(crate::source::repository::Error),
  ActivityRepositoryError(crate::activity::repository::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  source_repository: &mut impl SourceRepository,
  activity_repository: &mut impl ActivityRepository,
  input: Input,
) -> Result<Activity, Error> {
  let contribution: domain::source::vo::Contribution = match input.data {
    ChangeInput::Create(source) => domain::source::vo::Contribution::Create(source.into()),

    ChangeInput::Delete(id) => {
      let source = source_repository
        .find_by_id(&id)
        .await
        .map_err(Error::SourceRepositoryError)?;

      if let Some(source) = source {
        domain::source::vo::Contribution::Delete(source)
      } else {
        return Err(Error::SourceNotFound);
      }
    }
  };

  let activity = Activity::builder()
    .user_id(input.actor_id)
    .contribution(domain::activity::vo::Contribution::Source(contribution))
    .status(ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
