use crate::{domain, infra};

#[derive(Debug, serde::Deserialize)]
pub enum CreateActivityDto {
  Create(infra::actix::collaborative_entity::dto::CreateActivityEntityDTO),
  Update {
    entity_id: infra::actix::collaborative_entity::dto::CollaborativeEntityId,
    changes: infra::actix::collaborative_entity::dto::UpdateCollaborativeEntityDto,
  },
}

impl From<CreateActivityDto> for domain::activity::dto::CreateActivityDto {
  fn from(value: CreateActivityDto) -> Self {
    match value {
      CreateActivityDto::Create(dto) => {
        domain::activity::dto::CreateActivityDto::Create(dto.into())
      }
      CreateActivityDto::Update { entity_id, changes } => {
        domain::activity::dto::CreateActivityDto::Update {
          entity_id: entity_id.into(),
          changes: changes.into(),
        }
      }
    }
  }
}

#[derive(Debug, Clone, validator::Validate, serde::Deserialize)]
pub struct RejectActivityDto {
  pub activity_id: String,

  #[validate(length(min = 10, max = 255))]
  pub reason: String,
}
