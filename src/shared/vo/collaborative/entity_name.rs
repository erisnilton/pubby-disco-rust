use crate::infra::actix::collaborative_entity::dto::{CollaborativeEntity, CollaborativeEntityId};

#[derive(Debug, Clone)]
pub enum CollaborativeEntityName {
  Genre,
  Artist,
  Album,
}

impl From<CollaborativeEntity> for CollaborativeEntityName {
  fn from(value: CollaborativeEntity) -> Self {
    match value {
      CollaborativeEntity::Artist(_) => CollaborativeEntityName::Artist,
      CollaborativeEntity::Genre(_) => CollaborativeEntityName::Genre,
      CollaborativeEntity::Album(_) => CollaborativeEntityName::Album,
    }
  }
}

impl From<CollaborativeEntityId> for CollaborativeEntityName {
  fn from(value: CollaborativeEntityId) -> Self {
    match value {
      CollaborativeEntityId::Artist(_) => CollaborativeEntityName::Artist,
      CollaborativeEntityId::Genre(_) => CollaborativeEntityName::Genre,
      CollaborativeEntityId::Album(_) => CollaborativeEntityName::Album,
    }
  }
}

impl From<CollaborativeEntityName> for String {
  fn from(value: CollaborativeEntityName) -> Self {
    match value {
      CollaborativeEntityName::Album => "Album".to_string(),
      CollaborativeEntityName::Artist => "Artist".to_string(),
      CollaborativeEntityName::Genre => "Genre".to_string(),
    }
  }
}
