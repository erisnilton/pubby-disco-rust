use domain::{
  activity::entity::{Activity, ActivityStatus},
  album::entity::{Album, AlbumType},
};
use shared::vo::UUID4;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CreateInput {
  pub name: String,
  pub cover: Option<String>,
  pub album_type: AlbumType,
  pub release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub artist_ids: std::collections::HashSet<UUID4>,
}

impl From<CreateInput> for Album {
  fn from(value: CreateInput) -> Self {
    Self::builder()
      .name(value.name)
      .cover(value.cover)
      .album_type(value.album_type)
      .release_date(value.release_date)
      .parental_rating(value.parental_rating.unwrap_or_default())
      .artist_ids(value.artist_ids)
      .build()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateInput),
  Update {
    id: UUID4,
    changes: domain::album::vo::changes::Changes,
  },
  Delete(UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  AlbumRepositoryError(crate::album::repository::Error),
  ActivityRepositoryError(crate::activity::repository::Error),
  AlbumNotFound,
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  album_repository: &mut impl crate::album::repository::AlbumRepository,
  activity_repository: &mut impl crate::activity::repository::ActivityRepository,
  input: Input,
) -> Result<Activity, Error> {
  let contribution: domain::album::vo::Contribution = match input.data {
    ChangeInput::Create(album) => domain::album::vo::Contribution::Create(album.into()),
    ChangeInput::Delete(id) => {
      let album = album_repository
        .find_by_id(&id)
        .await
        .map_err(Error::AlbumRepositoryError)?;

      if let Some(album) = album {
        domain::album::vo::Contribution::Delete(album)
      } else {
        return Err(Error::AlbumNotFound);
      }
    }
    ChangeInput::Update { id, changes } => {
      let album = album_repository
        .find_by_id(&id)
        .await
        .map_err(Error::AlbumRepositoryError)?;

      if let Some(mut album) = album {
        album.apply_changes(&changes);
        domain::album::vo::Contribution::Update {
          entity: album,
          changes,
        }
      } else {
        return Err(Error::AlbumNotFound);
      }
    }
  };

  let activity = domain::activity::entity::Activity::builder()
    .user_id(input.actor_id)
    .contribution(domain::activity::vo::Contribution::Album(contribution))
    .status(ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
