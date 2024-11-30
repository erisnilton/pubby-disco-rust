use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct AlbumAggregate {
  album: super::Album,
  artists: Vec<crate::domain::artist::Artist>,
}

impl AlbumAggregate {
  pub fn new(album: super::Album, artists: Vec<crate::domain::artist::Artist>) -> Self {
    Self { album, artists }
  }

  pub fn into_parts(self) -> (super::Album, Vec<crate::domain::artist::Artist>) {
    (self.album, self.artists)
  }
}
