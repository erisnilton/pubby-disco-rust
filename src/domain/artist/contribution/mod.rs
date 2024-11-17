pub mod changes;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Create(crate::domain::artist::Artist),
  Update {
    entity: crate::domain::artist::Artist,
    changes: changes::Changes,
  },
  Delete(crate::domain::artist::Artist),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Create(crate::domain::artist::Artist::default())
  }
}
