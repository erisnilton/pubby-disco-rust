use super::entity::Source;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Source),
  Delete(Source),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Source::default())
  }
}
