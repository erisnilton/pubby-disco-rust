use domain::{
  activity::{
    entity::{Activity, ActivityStatus},
    vo::Contribution,
  },
  artist::entity::Artist,
};
use shared::vo::{Slug, UUID4};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CreateArtistInput {
  pub name: String,
  pub slug: Option<Slug>,
  pub country: Option<String>,
}

impl From<CreateArtistInput> for Artist {
  fn from(value: CreateArtistInput) -> Self {
    Self::builder()
      .name(value.name.clone())
      .slug(value.slug.unwrap_or_else(|| Slug::generate(&value.name)))
      .country(value.country.clone())
      .build()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateArtistInput),
  Update {
    id: UUID4,
    changes: domain::artist::vo::changes::Changes,
  },
  Delete(UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  ArtistRepositoryError(crate::artist::repository::Error),
  ActivityRepositoryError(crate::activity::repository::Error),
  ArtistNotFound,
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  artist_repository: &mut impl crate::artist::repository::ArtistRepository,
  activity_repository: &mut impl crate::activity::repository::ActivityRepository,
  input: Input,
) -> Result<domain::activity::entity::Activity, Error> {
  let contribution: domain::artist::vo::Contribution = match input.data {
    ChangeInput::Create(artist) => domain::artist::vo::Contribution::Create(artist.into()),
    ChangeInput::Delete(id) => {
      let artist = artist_repository
        .find_by_id(&id)
        .await
        .map_err(Error::ArtistRepositoryError)?;

      if let Some(artist) = artist {
        domain::artist::vo::Contribution::Delete(artist)
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
        domain::artist::vo::Contribution::Update {
          entity: artist,
          changes,
        }
      } else {
        return Err(Error::ArtistNotFound);
      }
    }
  };

  let activity = Activity::builder()
    .user_id(input.actor_id)
    .contribution(Contribution::Artist(contribution))
    .status(ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
