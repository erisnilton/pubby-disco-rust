use std::vec;

use super::entity::Genre;

#[derive(Debug, Clone, PartialEq)]
pub struct GenreAggregate {
  genre: Genre,
  related: vec::Vec<Genre>,
}

impl GenreAggregate {
  pub fn new(genre: Genre, releated: Vec<Genre>) -> Self {
    Self {
      genre,
      related: releated,
    }
  }

  pub fn genre(&self) -> &Genre {
    &self.genre
  }

  pub fn related(&self) -> &Vec<Genre> {
    &self.related
  }
}
