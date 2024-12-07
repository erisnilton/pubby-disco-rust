use domain::{
  activity::entity::{Activity, ActivityStatus},
  media::entity::{Media, MediaBuilder, MediaType},
};
use shared::vo::{Slug, UUID4};

use crate::{activity::repository::ActivityRepository, media::repository::MediaRepository};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMediaInput {
  pub name: String,

  pub media_type: MediaType,

  pub slug: Option<Slug>,

  pub release_date: Option<chrono::NaiveDate>,

  pub cover: Option<String>,

  pub genre_ids: Option<std::collections::HashSet<UUID4>>,

  pub parental_rating: Option<u8>,

  pub composer_ids: Option<std::collections::HashSet<UUID4>>,

  pub interpreter_ids: std::collections::HashSet<UUID4>,

  pub album_ids: std::collections::HashSet<UUID4>,

  pub is_single: Option<bool>,
}

impl From<CreateMediaInput> for Media {
  fn from(value: CreateMediaInput) -> Self {
    MediaBuilder::new()
      .name(value.name.clone())
      .media_type(value.media_type)
      .slug(
        value
          .slug
          .clone()
          .unwrap_or_else(|| Slug::generate(&value.name)),
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
    id: UUID4,
    changes: domain::media::vo::Changes,
  },
  Delete(UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  MediaNotFound,
  MediaRepositoryError(crate::media::repository::Error),
  ActivityRepositoryError(crate::activity::repository::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  media_repository: &mut impl MediaRepository,
  activity_repository: &mut impl ActivityRepository,
  input: Input,
) -> Result<Activity, Error> {
  let contribution: domain::media::vo::Contribution = match input.data {
    ChangeInput::Create(media) => domain::media::vo::Contribution::Create(media.into()),

    ChangeInput::Delete(id) => {
      let media = media_repository
        .find_by_id(&id)
        .await
        .map_err(Error::MediaRepositoryError)?;

      if let Some(media) = media {
        domain::media::vo::Contribution::Delete(media)
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
        domain::media::vo::Contribution::Update {
          entity: media,
          changes,
        }
      } else {
        return Err(Error::MediaNotFound);
      }
    }
  };

  let activity = Activity::builder()
    .user_id(input.actor_id)
    .contribution(domain::activity::vo::Contribution::Media(contribution))
    .status(ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
