mod changes;

pub use changes::Changes;

use super::entity::Genre;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Genre),
  Update { entity: Genre, changes: Changes },
  Delete(Genre),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Genre::default())
  }
}
