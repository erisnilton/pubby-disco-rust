#[derive(Debug, Clone, validator::Validate, serde::Deserialize)]
pub struct RejectActivityDto {
  #[validate(length(min = 10, max = 255))]
  pub reason: String,
}
