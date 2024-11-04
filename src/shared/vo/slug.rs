use core::fmt;
use std::fmt::{Display, Formatter};

use slug::slugify;
use unidecode::unidecode;

#[derive(Debug)]
pub enum SlugError {
  InvalidSlug,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Slug(pub String);

impl Slug {
  pub fn new(slug: &str) -> Result<Self, SlugError> {
    if !slug.is_empty()
      && slug
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_'))
    {
      Ok(Self(slug.to_string()))
    } else {
      Err(SlugError::InvalidSlug)
    }
  }
  pub fn generate(text: &str) -> Self {
    Self(slugify(unidecode(text)))
  }
}

impl Display for Slug {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
