use crate::{
  domain::{activity::ActivityChange, genre::GenreRepositoryError},
  shared::vo::{CollaborativeEntity, UpdateCollaborativeEntityDto},
};

#[derive(Debug, Clone)]
pub enum ApplyChangesError {
  RepositoryError(GenreRepositoryError),
  EntityIsNotGenre,
}

pub type Input = ActivityChange;

pub async fn execute(
  repository_genre: &mut impl crate::domain::genre::GenreRepository,
  input: Input,
) -> Result<(), ApplyChangesError> {
  match input {
    ActivityChange::Create(entity) => match entity {
      crate::shared::vo::CollaborativeEntity::Genre(genre) => {
        repository_genre
          .create(genre)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
        Ok(())
      }
      _ => Err(ApplyChangesError::EntityIsNotGenre),
    },
    ActivityChange::Update {
      entity,
      old_value,
      new_value,
    } => match (entity, old_value, new_value) {
      (
        CollaborativeEntity::Genre(mut genre),
        UpdateCollaborativeEntityDto::Genre(old_value),
        UpdateCollaborativeEntityDto::Genre(new_value),
      ) => {
        genre.apply_changes(&old_value, &new_value);
        repository_genre
          .update(genre)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
        Ok(())
      }
      _ => Err(ApplyChangesError::EntityIsNotGenre),
    },
    ActivityChange::Delete(entity) => match entity {
      CollaborativeEntity::Genre(genre) => {
        repository_genre
          .delete_by_id(&genre.id)
          .await
          .map_err(ApplyChangesError::RepositoryError)?;
        Ok(())
      }
      _ => Err(ApplyChangesError::EntityIsNotGenre),
    },
  }
}
