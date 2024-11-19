#[derive(Debug, Clone)]
pub enum Error {
  UserIsNotACurator,
  ActivityNotFound,
  RepositoryError(crate::domain::activity::repository::Error),

  ActivityError(crate::domain::activity::Error),

  GenreApplyError(crate::domain::genre::stories::apply_changes::Error),
  ArtistApplyError(crate::domain::artist::stories::apply_changes::Error),
  AlbumApplyError(crate::domain::album::stories::apply_changes::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub activity_id: crate::shared::vo::UUID4,
  pub actor: crate::domain::user::User,
}

pub async fn execute(
  activity_repository: &mut impl crate::domain::activity::repository::ActivityRepository,
  genre_repository: &mut impl crate::domain::genre::GenreRepository,
  artist_repository: &mut impl crate::domain::artist::repository::ArtistRepository,
  album_repository: &mut impl crate::domain::album::repository::AlbumRepository,
  input: Input,
) -> Result<crate::domain::activity::Activity, Error> {
  if !input.actor.is_curator() {
    return Err(Error::UserIsNotACurator);
  }

  let activity = activity_repository
    .find_by_id(&input.activity_id)
    .await
    .map_err(Error::RepositoryError)?;

  if let Some(mut activity) = activity {
    activity = activity
      .set_curator_status(
        crate::domain::activity::ActivityStatus::Approved,
        input.actor.id(),
      )
      .map_err(Error::ActivityError)?;

    match activity.contribution() {
      crate::shared::vo::Contribution::Genre(contribution) => {
        crate::domain::genre::stories::apply_changes::execute(
          genre_repository,
          contribution.clone(),
        )
        .await
        .map_err(Error::GenreApplyError)?;
      }
      crate::shared::vo::Contribution::Artist(contribution) => {
        crate::domain::artist::stories::apply_changes::execute(
          artist_repository,
          contribution.clone(),
        )
        .await
        .map_err(Error::ArtistApplyError)?;
      }
      crate::shared::vo::Contribution::Album(contribution) => {
        crate::domain::album::stories::apply_changes::execute(
          album_repository,
          contribution.clone(),
        )
        .await
        .map_err(Error::AlbumApplyError)?;
      }
    }

    return Ok(activity);
  }
  Err(Error::ActivityNotFound)
}
