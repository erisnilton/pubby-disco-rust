#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CreateInput {
  pub name: String,
  pub cover: Option<String>,
  pub release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub artist_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
}

impl From<CreateInput> for crate::domain::album::Album {
  fn from(value: CreateInput) -> Self {
    Self {
      name: value.name.clone(),
      cover: value.cover.clone(),
      release_date: value.release_date,
      parental_rating: value.parental_rating,
      artist_ids: value.artist_ids.clone(),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateInput),
  Update {
    id: crate::shared::vo::UUID4,
    changes: crate::domain::album::contribution::changes::Changes,
  },
  Delete(crate::shared::vo::UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  AlbumRepositoryError(crate::domain::album::repository::Error),
  ActivityRepositoryError(crate::domain::activity::repository::Error),
  AlbumNotFound,
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: crate::shared::vo::UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  album_repository: &mut impl crate::domain::album::repository::AlbumRepository,
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  let contribution: crate::domain::album::contribution::Contribution = match input.data {
    ChangeInput::Create(album) => {
      crate::domain::album::contribution::Contribution::Create(album.into())
    }

    ChangeInput::Delete(id) => {
      let album = album_repository
        .find_by_id(&id)
        .await
        .map_err(Error::AlbumRepositoryError)?;

      if let Some(album) = album {
        crate::domain::album::contribution::Contribution::Delete(album)
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
        crate::domain::album::contribution::Contribution::Update {
          entity: album,
          changes,
        }
      } else {
        return Err(Error::AlbumNotFound);
      }
    }
  };

  let activity = crate::domain::activity::Activity {
    user_id: input.actor_id,
    contribuition: crate::shared::vo::Contribution::Album(contribution),
    status: crate::domain::activity::ActivityStatus::Pending,
    ..Default::default()
  };

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
