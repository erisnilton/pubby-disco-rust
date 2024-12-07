use super::entity::Artist;

pub mod changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Artist),
  Update {
    entity: Artist,
    changes: changes::Changes,
  },
  Delete(Artist),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Artist::default())
  }
}
