use super::entity::*;
use crate::artist::entity::Artist;

#[derive(Debug, Clone, PartialEq)]
pub struct AlbumAggregate {
  album: Album,
  artists: Vec<Artist>,
}

impl AlbumAggregate {
  pub fn new(album: Album, artists: Vec<Artist>) -> Self {
    Self { album, artists }
  }

  pub fn into_parts(self) -> (Album, Vec<Artist>) {
    (self.album, self.artists)
  }
}
