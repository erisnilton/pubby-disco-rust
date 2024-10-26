use crate::{
  domain::genre::dto::CreateGenreDto,
  shared::vo::{CollaborativeEntityId, UpdateCollaborativeEntityDto},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CreateActivityEntityDto {
  Genre(CreateGenreDto),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum CreateActivityDto {
  Create(CreateActivityEntityDto),
  Update {
    entity_id: CollaborativeEntityId,
    changes: UpdateCollaborativeEntityDto,
  },
}
