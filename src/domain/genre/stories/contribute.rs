use crate::{domain::genre::GenreBuilder, shared::vo::Slug};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateGenreInput {
  pub name: String,

  pub slug: Option<Slug>,

  pub parent_id: Option<crate::shared::vo::UUID4>,
}

impl From<CreateGenreInput> for crate::domain::genre::Genre {
  fn from(value: CreateGenreInput) -> Self {
    GenreBuilder::default()
      .name(value.name.clone())
      .slug(value.slug.unwrap_or_else(|| Slug::generate(&value.name)))
      .parent_id(value.parent_id)
      .build()
      .unwrap()
  }
}

#[derive(Debug, Clone)]
pub enum ChangeInput {
  Create(CreateGenreInput),
  Update {
    id: crate::shared::vo::UUID4,
    changes: crate::domain::genre::contribution::changes::Changes,
  },
  Delete(crate::shared::vo::UUID4),
}

#[derive(Debug, Clone)]
pub enum Error {
  GenreNotFound,
  GenreRepositoryError(crate::domain::genre::Error),
  ActivityRepositoryError(crate::domain::activity::repository::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub actor_id: crate::shared::vo::UUID4,
  pub data: ChangeInput,
}

pub async fn execute(
  genre_repository: &mut impl crate::domain::genre::repository::GenreRepository,
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  let contribution: crate::domain::genre::contribution::Contribution = match input.data {
    ChangeInput::Create(genre) => {
      crate::domain::genre::contribution::Contribution::Create(genre.into())
    }

    ChangeInput::Delete(id) => {
      let genre = genre_repository
        .find_by_id(&id)
        .await
        .map_err(Error::GenreRepositoryError)?;

      if let Some(genre) = genre {
        crate::domain::genre::contribution::Contribution::Delete(genre)
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
        crate::domain::genre::contribution::Contribution::Update {
          entity: genre,
          changes,
        }
      } else {
        return Err(Error::GenreNotFound);
      }
    }
  };

  let activity = crate::domain::activity::Activity {
    user_id: input.actor_id,
    contribuition: crate::shared::vo::Contribution::Genre(contribution),
    status: crate::domain::activity::ActivityStatus::Pending,
    ..Default::default()
  };

  activity_repository
    .create(&activity)
    .await
    .map_err(Error::ActivityRepositoryError)?;

  Ok(activity)
}
