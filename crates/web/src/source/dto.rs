use application::source::stories::contribute::CreateSourceInput;
use domain::source::entity::{Source, SourceType};
use shared::vo::UUID4;

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateSourceDTO {
  #[validate(length(min = 1))]
  pub src: String,
  pub media_id: shared::vo::UUID4,
  pub source_type: SourceType,
}
impl From<CreateSourceDTO> for CreateSourceInput {
  fn from(value: CreateSourceDTO) -> Self {
    Self {
      src: value.src,
      media_id: value.media_id,
      source_type: value.source_type,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct SourcePresenter {
  id: UUID4,
  source_type: SourceType,
  src: String,
}

impl From<Source> for SourcePresenter {
  fn from(value: Source) -> Self {
    Self {
      id: value.id().clone(),
      source_type: value.source_type().clone(),
      src: value.src().clone(),
    }
  }
}
