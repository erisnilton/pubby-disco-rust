mod changes;

use super::entity::Media;
pub use changes::Changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Media),
  Update { entity: Media, changes: Changes },
  Delete(Media),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Media::default())
  }
}
