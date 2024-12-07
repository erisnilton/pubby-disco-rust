use domain::{
  activity::{
    entity::{Activity, ActivityStatus},
    vo::Contribution,
  },
  user::entity::User,
};
use shared::vo::UUID4;

#[derive(Debug, Clone)]
pub enum Error {
  UserIsNotACurator,
  ActivityNotFound,
  RepositoryError(crate::activity::repository::Error),

  ActivityError(domain::activity::entity::Error),

  GenreApplyError(crate::genre::stories::apply_changes::Error),
  ArtistApplyError(crate::artist::stories::apply_changes::Error),
  AlbumApplyError(crate::album::stories::apply_changes::Error),
  MediaApplyError(crate::media::stories::apply_changes::Error),
  SourceApplyError(crate::source::stories::apply_changes::Error),
}

#[derive(Debug, Clone)]
pub struct Input {
  pub activity_id: UUID4,
  pub actor: User,
}

pub async fn execute(
  activity_repository: &mut impl crate::activity::repository::ActivityRepository,
  genre_repository: &mut impl crate::genre::repository::GenreRepository,
  artist_repository: &mut impl crate::artist::repository::ArtistRepository,
  album_repository: &mut impl crate::album::repository::AlbumRepository,
  media_repository: &mut impl crate::media::repository::MediaRepository,
  source_repository: &mut impl crate::source::repository::SourceRepository,
  input: Input,
) -> Result<Activity, Error> {
  if !input.actor.is_curator() {
    return Err(Error::UserIsNotACurator);
  }

  let activity = activity_repository
    .find_by_id(&input.activity_id)
    .await
    .map_err(Error::RepositoryError)?;

  if let Some(mut activity) = activity {
    activity = activity
      .set_curator_status(ActivityStatus::Approved, input.actor.id())
      .map_err(Error::ActivityError)?;

    match activity.contribution() {
      Contribution::Genre(contribution) => {
        crate::genre::stories::apply_changes::execute(genre_repository, contribution.clone())
          .await
          .map_err(Error::GenreApplyError)?;
      }
      Contribution::Artist(contribution) => {
        crate::artist::stories::apply_changes::execute(artist_repository, contribution.clone())
          .await
          .map_err(Error::ArtistApplyError)?;
      }
      Contribution::Album(contribution) => {
        crate::album::stories::apply_changes::execute(album_repository, contribution.clone())
          .await
          .map_err(Error::AlbumApplyError)?;
      }
      Contribution::Media(contribution) => {
        crate::media::stories::apply_changes::execute(
          media_repository,
          album_repository,
          contribution.clone(),
        )
        .await
        .map_err(Error::MediaApplyError)?;
      }
      Contribution::Source(contribution) => {
        crate::source::stories::apply_changes::execute(source_repository, contribution.clone())
          .await
          .map_err(Error::SourceApplyError)?;
      }
    }

    return Ok(activity);
  }
  Err(Error::ActivityNotFound)
}
