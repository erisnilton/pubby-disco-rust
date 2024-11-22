use crate::*;

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateSourceDTO {
  #[validate(length(min = 1))]
  pub src: String,
  pub media_id: shared::vo::UUID4,
  pub source_type: domain::source::source_entity::SourceType,
}
impl From<CreateSourceDTO> for crate::domain::source::stories::contribute::CreateSourceInput {
  fn from(value: infra::actix::source::dto::CreateSourceDTO) -> Self {
    Self {
      src: value.src,
      media_id: value.media_id,
      source_type: value.source_type,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct SourcePresenter {
  id: crate::shared::vo::UUID4,
  source_type: domain::source::source_entity::SourceType,
  src: String,
}

impl From<domain::source::source_entity::Source> for SourcePresenter {
  fn from(value: domain::source::source_entity::Source) -> Self {
    Self {
      id: value.id().clone(),
      source_type: value.source_type().clone(),
      src: value.src().clone(),
    }
  }
}
