use super::entity::Album;

pub mod changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Album),
  Update {
    entity: Album,
    changes: changes::Changes,
  },
  Delete(Album),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Album::default())
  }
}
