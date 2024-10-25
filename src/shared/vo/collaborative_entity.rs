use core::panic;

use serde::{Deserialize, Serialize};

use crate::domain::genre::{dto::UpdateGenreDto, Genre};

use super::UUID4;

#[derive(Debug, Clone)]
pub enum CollaborativeEntity {
  Default,
  Genre(Genre),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UpdateCollaborativeEntityDto {
  Default,
  Genre(UpdateGenreDto),
}

impl CollaborativeEntity {
  pub fn id(&self) -> String {
    match self {
      CollaborativeEntity::Genre(genre) => genre.id.0.clone(),
      CollaborativeEntity::Default => panic!("CollaborativeEntity::Default não possui id!"),
    }
  }
  pub fn name(&self) -> String {
    match self {
      CollaborativeEntity::Genre(..) => "Genre".to_string(),
      CollaborativeEntity::Default => panic!("CollaborativeEntity::Default não possui name!"),
    }
  }
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub enum CollaborativeEntityId {
  Default,
  Genre(UUID4),
}

pub trait GetChanges<T> {
  fn get_changes(&self, changes: T) -> (T, T);
}

pub trait IntoRecord {
  fn into(&self) -> serde_json::Value;
}

impl IntoRecord for CollaborativeEntity {
  fn into(&self) -> serde_json::Value {
    match self {
      CollaborativeEntity::Genre(genre) => IntoRecord::into(genre),
      value => panic!("IntoRecord não implementado para o valor: {:#?}", value),
    }
  }
}

impl IntoRecord for UpdateCollaborativeEntityDto {
  fn into(&self) -> serde_json::Value {
    match self {
      UpdateCollaborativeEntityDto::Genre(genre_dto) => IntoRecord::into(genre_dto),
      value => panic!("IntoRecord não implementado para o valor: {:#?}", value),
    }
  }
}

impl GetChanges<UpdateCollaborativeEntityDto> for CollaborativeEntity {
  fn get_changes(
    &self,
    changes: UpdateCollaborativeEntityDto,
  ) -> (UpdateCollaborativeEntityDto, UpdateCollaborativeEntityDto) {
    if let CollaborativeEntity::Genre(genre) = self {
      if let UpdateCollaborativeEntityDto::Genre(genre_changes) = changes {
        let (old_value, new_value) = genre.get_changes(genre_changes);

        return (
          UpdateCollaborativeEntityDto::Genre(old_value),
          UpdateCollaborativeEntityDto::Genre(new_value),
        );
      }

      panic!(
        "Era esperado um UpdateCollaborativeEntityDto::Genre, mas foi recebido: {:#?}",
        changes
      );
    }

    panic!(
      "Era esperado um CollaborativeEntity, mas foi recebido: {:#?}",
      self
    );
  }
}

impl Default for CollaborativeEntity {
  fn default() -> Self {
    Self::Default
  }
}
