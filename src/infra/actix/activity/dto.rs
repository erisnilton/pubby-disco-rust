#[derive(Debug, Clone, validator::Validate, serde::Deserialize)]
pub struct RejectActivityDto {
  pub activity_id: String,

  #[validate(length(min = 10, max = 255))]
  pub reason: String,
}

#[derive(Debug, Clone, validator::Validate, serde::Deserialize)]
pub struct ApproveActivityDto {
  pub activity_id: String,
}
