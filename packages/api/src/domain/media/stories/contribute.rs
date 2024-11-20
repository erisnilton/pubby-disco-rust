#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMediaInput {
  pub name: String,

  pub media_type: crate::domain::media::MediaType,

  pub slug: Option<crate::shared::vo::Slug>,

  pub release_date: Option<chrono::NaiveDate>,

  pub cover: Option<String>,

  pub genre_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub parental_rating: Option<u8>,

  pub composer_ids: Option<std::collections::HashSet<crate::shared::vo::UUID4>>,

  pub interpreter_ids: std::collections::HashSet<crate::shared::vo::UUID4>,

  pub album_ids: std::collections::HashSet<crate::shared::vo::UUID4>,

  pub is_single: Option<bool>,
}

impl From<CreateMediaInput> for crate::domain::media::Media {
  fn from(value: CreateMediaInput) -> Self {
    crate::domain::media::MediaBuilder::new()
      .name(value.name.clone())
      .media_type(value.media_type)
      .slug(
        value
          .slug
          .clone()
          .unwrap_or_else(|| crate::shared::vo::Slug::generate(&value.name)),
      )
      .release_date(value.release_date)
      .cover(value.cover)
      .genre_ids(value.genre_ids.unwrap_or_default())
      .parental_rating(value.parental_rating.unwrap_or_default())
      .composer_ids(value.composer_ids.unwrap_or_default())
      .interpreter_ids(value.interpreter_ids)
      .album_ids(value.album_ids)
      .is_single(value.is_single.unwrap_or_default())
      .build()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateMediaInput),
  Update {
    id: crate::shared::vo::UUID4,
    changes: crate::domain::media::contribution::changes::Changes,
  },
  Delete(crate::shared::vo::UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  MediaNotFound,
  MediaRepositoryError(crate::domain::media::repository::Error),
  ActivityRepositoryError(crate::domain::activity::repository::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: crate::shared::vo::UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  media_repository: &mut impl crate::domain::media::repository::MediaRepository,
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  let contribution: crate::domain::media::contribution::Contribution = match input.data {
    ChangeInput::Create(media) => {
      crate::domain::media::contribution::Contribution::Create(media.into())
    }

    ChangeInput::Delete(id) => {
      let media = media_repository
        .find_by_id(&id)
        .await
        .map_err(Error::MediaRepositoryError)?;

      if let Some(media) = media {
        crate::domain::media::contribution::Contribution::Delete(media)
      } else {
        return Err(Error::MediaNotFound);
      }
    }

    ChangeInput::Update { id, changes } => {
      let media = media_repository
        .find_by_id(&id)
        .await
        .map_err(Error::MediaRepositoryError)?;

      if let Some(media) = media {
        crate::domain::media::contribution::Contribution::Update {
          entity: media,
          changes,
        }
      } else {
        return Err(Error::MediaNotFound);
      }
    }
  };

  let activity = crate::domain::activity::Activity::builder()
    .user_id(input.actor_id)
    .contribution(crate::shared::vo::Contribution::Media(contribution))
    .status(crate::domain::activity::ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
