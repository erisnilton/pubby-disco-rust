#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(super::source_entity::Source),
  Delete(super::source_entity::Source),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(super::source_entity::Source::default())
  }
}
