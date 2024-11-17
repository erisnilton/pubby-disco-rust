use crate::domain;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CollaborativeEntityName {
  Genre,
  Artist,
  Album,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Genre(domain::genre::contribution::Contribution),
  Artist(domain::artist::contribution::Contribution),
  Album(domain::album::contribution::Contribution),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Genre(domain::genre::contribution::Contribution::default())
  }
}
