use super::Genre;

pub mod changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(Genre),
  Update {
    entity: Genre,
    changes: changes::Changes,
  },
  Delete(Genre),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(Genre::default())
  }
}
