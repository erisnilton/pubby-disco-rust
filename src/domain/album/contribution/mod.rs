pub mod changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(crate::domain::album::Album),
  Update {
    entity: crate::domain::album::Album,
    changes: changes::Changes,
  },
  Delete(crate::domain::album::Album),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(crate::domain::album::Album::default())
  }
}
