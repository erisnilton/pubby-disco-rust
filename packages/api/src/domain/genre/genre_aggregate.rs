use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct GenreAggregate {
  genre: super::Genre,
  related: vec::Vec<super::Genre>,
}

impl GenreAggregate {
  pub fn new(genre: super::Genre, releated: Vec<super::Genre>) -> Self {
    Self {
      genre,
      related: releated,
    }
  }

  pub fn genre(&self) -> &super::Genre {
    &self.genre
  }

  pub fn related(&self) -> &Vec<super::Genre> {
    &self.related
  }
}
