use domain::{
  activity::{
    entity::{Activity, ActivityStatus},
    vo::Contribution,
  },
  genre::entity::{Genre, GenreBuilder},
};
use shared::vo::{Slug, UUID4};

use crate::{activity::repository::ActivityRepository, genre::repository::GenreRepository};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateGenreInput {
  pub name: String,

  pub slug: Option<Slug>,

  pub parent_id: Option<UUID4>,
}

impl From<CreateGenreInput> for Genre {
  fn from(value: CreateGenreInput) -> Self {
    GenreBuilder::new()
      .name(value.name.clone())
      .slug(value.slug.unwrap_or_else(|| Slug::generate(&value.name)))
      .parent_id(value.parent_id)
      .build()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateGenreInput),
  Update {
    id: UUID4,
    changes: domain::genre::vo::Changes,
  },
  Delete(UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  GenreNotFound,
  GenreRepositoryError(crate::genre::repository::Error),
  ActivityRepositoryError(crate::activity::repository::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  genre_repository: &mut impl GenreRepository,
  activity_repository: &mut impl ActivityRepository,
  input: Input,
) -> Result<Activity, Error> {
  let contribution: domain::genre::vo::Contribution = match input.data {
    ChangeInput::Create(genre) => domain::genre::vo::Contribution::Create(genre.into()),

    ChangeInput::Delete(id) => {
      let genre = genre_repository
        .find_by_id(&id)
        .await
        .map_err(Error::GenreRepositoryError)?;

      if let Some(genre) = genre {
        domain::genre::vo::Contribution::Delete(genre)
      } else {
        return Err(Error::GenreNotFound);
      }
    }

    ChangeInput::Update { id, changes } => {
      let genre = genre_repository
        .find_by_id(&id)
        .await
        .map_err(Error::GenreRepositoryError)?;

      if let Some(genre) = genre {
        domain::genre::vo::Contribution::Update {
          entity: genre,
          changes,
        }
      } else {
        return Err(Error::GenreNotFound);
      }
    }
  };

  let activity = Activity::builder()
    .user_id(input.actor_id)
    .contribution(Contribution::Genre(contribution))
    .status(ActivityStatus::Pending)
    .build();

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
