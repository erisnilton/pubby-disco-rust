use super::Media;

pub mod changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Media),
  Update {
    entity: Media,
    changes: changes::Changes,
  },
  Delete(Media),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Media::default())
  }
}
