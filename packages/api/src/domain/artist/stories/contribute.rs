use crate::shared::vo::Slug;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CreateArtistInput {
  pub name: String,
  pub slug: Option<Slug>,
  pub country: Option<String>,
}

impl From<CreateArtistInput> for crate::domain::artist::Artist {
  fn from(value: CreateArtistInput) -> Self {
    Self {
      name: value.name.clone(),
      slug: value
        .slug
        .unwrap_or_else(|| crate::shared::vo::Slug::generate(&value.name)),
      country: value.country.clone(),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateArtistInput),
  Update {
    id: crate::shared::vo::UUID4,
    changes: crate::domain::artist::contribution::changes::Changes,
  },
  Delete(crate::shared::vo::UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  ArtistRepositoryError(crate::domain::artist::repository::Error),
  ActivityRepositoryError(crate::domain::activity::repository::Error),
  ArtistNotFound,
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: crate::shared::vo::UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  artist_repository: &mut impl crate::domain::artist::repository::ArtistRepository,
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  let contribution: crate::domain::artist::contribution::Contribution = match input.data {
    ChangeInput::Create(artist) => {
      crate::domain::artist::contribution::Contribution::Create(artist.into())
    }

    ChangeInput::Delete(id) => {
      let artist = artist_repository
        .find_by_id(&id)
        .await
        .map_err(Error::ArtistRepositoryError)?;

      if let Some(artist) = artist {
        crate::domain::artist::contribution::Contribution::Delete(artist)
      } else {
        return Err(Error::ArtistNotFound);
      }
    }

    ChangeInput::Update { id, changes } => {
      let artist = artist_repository
        .find_by_id(&id)
        .await
        .map_err(Error::ArtistRepositoryError)?;

      if let Some(mut artist) = artist {
        artist.apply_changes(&changes);
        crate::domain::artist::contribution::Contribution::Update {
          entity: artist,
          changes,
        }
      } else {
        return Err(Error::ArtistNotFound);
      }
    }
  };

  let activity = crate::domain::activity::Activity::builder()
    .user_id(input.actor_id)
    .contribution(crate::shared::vo::Contribution::Artist(contribution))
    .status(crate::domain::activity::ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
