#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateSourceInput {
  pub source_type: crate::domain::source::source_entity::SourceType,
  pub src: String,
  pub media_id: crate::shared::vo::UUID4,
}

impl From<CreateSourceInput> for crate::domain::source::source_entity::Source {
  fn from(value: CreateSourceInput) -> Self {
    crate::domain::source::source_entity::Source::builder()
      .source_type(value.source_type)
      .src(value.src)
      .media_id(value.media_id)
      .build()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateSourceInput),
  Delete(crate::shared::vo::UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  SourceNotFound,
  SourceRepositoryError(crate::domain::source::repository::Error),
  ActivityRepositoryError(crate::domain::activity::repository::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: crate::shared::vo::UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  source_repository: &mut impl crate::domain::source::repository::SourceRepository,
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  let contribution: crate::domain::source::contribution::Contribution = match input.data {
    ChangeInput::Create(source) => {
      crate::domain::source::contribution::Contribution::Create(source.into())
    }

    ChangeInput::Delete(id) => {
      let source = source_repository
        .find_by_id(&id)
        .await
        .map_err(Error::SourceRepositoryError)?;

      if let Some(source) = source {
        crate::domain::source::contribution::Contribution::Delete(source)
      } else {
        return Err(Error::SourceNotFound);
      }
    }
  };

  let activity = crate::domain::activity::Activity::builder()
    .user_id(input.actor_id)
    .contribution(crate::shared::vo::Contribution::Source(contribution))
    .status(crate::domain::activity::ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
