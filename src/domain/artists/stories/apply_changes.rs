use crate::{
  domain::{activity::ActivityChange, artists::repository::ArtistRepositoryError},
  shared::vo::{CollaborativeEntity, UpdateCollaborativeEntity},
};

#[derive(Debug, Clone)]
pub enum ApplyChangesError {
  RepositoryError(ArtistRepositoryError),
  EntityIsNotArtist,
}

pub type Input = ActivityChange;

pub async fn execute(
  repository_artist: &mut impl crate::domain::artists::repository::ArtistRepository,
  input: Input,
) -> Result<(), ApplyChangesError> {
  match input {
    ActivityChange::Create(entity) => match entity {
      crate::shared::vo::CollaborativeEntity::Artist(artist) => {
        repository_artist
          .create(&artist)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
        Ok(())
      }
      _ => Err(ApplyChangesError::EntityIsNotArtist),
    },
    ActivityChange::Update {
      entity,
      old_value,
      new_value,
    } => match (entity, old_value, new_value) {
      (
        CollaborativeEntity::Artist(mut artist),
        UpdateCollaborativeEntity::Artist(old_value),
        UpdateCollaborativeEntity::Artist(new_value),
      ) => {
        artist.apply_changes(&old_value, &new_value);
        repository_artist
          .update(&artist)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
        Ok(())
      }
      _ => Err(ApplyChangesError::EntityIsNotArtist),
    },
    ActivityChange::Delete(entity) => match entity {
      CollaborativeEntity::Artist(artist) => {
        repository_artist
          .delete_by_id(&artist.id)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
        Ok(())
      }
      _ => Err(ApplyChangesError::EntityIsNotArtist),
    },
  }
}
