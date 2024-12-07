#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CollaborativeEntityName {
  Genre,
  Artist,
  Album,
  Media,
  Source,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Contribution {
  Genre(crate::genre::vo::Contribution),
  Artist(crate::artist::vo::Contribution),
  Album(crate::album::vo::Contribution),
  Media(crate::media::vo::Contribution),
  Source(crate::source::vo::Contribution),
}

impl Default for Contribution {
  fn default() -> Self {
    Self::Genre(crate::genre::vo::Contribution::default())
  }
}
