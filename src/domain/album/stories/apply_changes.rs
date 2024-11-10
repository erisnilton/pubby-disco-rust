use crate::{
  domain::{activity::ActivityChange, album::AlbumRepositoryError},
  shared::vo::UpdateCollaborativeEntity,
};

#[derive(Debug, Clone)]
pub enum ApplyChangesError {
  RepositoryError(AlbumRepositoryError),
  EntityIsNotAlbum,
}

pub type Input = ActivityChange;

pub async fn execute(
  repository_album: &mut impl crate::domain::album::AlbumRepository,
  input: Input,
) -> Result<(), ApplyChangesError> {
  match input {
    ActivityChange::Create(entity) => match entity {
      crate::shared::vo::CollaborativeEntity::Album(album) => {
        repository_album
          .create(album)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
      }
      _ => return Err(ApplyChangesError::EntityIsNotAlbum),
    },
    ActivityChange::Update {
      entity,
      old_value,
      new_value,
    } => match (entity, old_value, new_value) {
      (
        crate::shared::vo::CollaborativeEntity::Album(mut album),
        UpdateCollaborativeEntity::Album(old_value),
        UpdateCollaborativeEntity::Album(new_value),
      ) => {
        album.apply_changes(&old_value, &new_value);
        repository_album
          .update(album)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
      }
      _ => return Err(ApplyChangesError::EntityIsNotAlbum),
    },
    ActivityChange::Delete(entity) => match entity {
      crate::shared::vo::CollaborativeEntity::Album(album) => {
        repository_album
          .delete_by_id(&album.id)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
      }
      _ => return Err(ApplyChangesError::EntityIsNotAlbum),
    },
  }
  Ok(())
}
