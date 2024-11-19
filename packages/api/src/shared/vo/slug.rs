use core::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use slug::slugify;
use unidecode::unidecode;

#[derive(Debug)]
pub enum SlugError {
  InvalidSlug,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Slug(pub String);

impl Slug {
  pub fn new(slug: impl Into<String>) -> Result<Self, SlugError> {
    let slug = slug.into();

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

impl Serialize for Slug {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.0)
  }
}

impl<'de> Deserialize<'de> for Slug {
  fn deserialize<D>(deserializer: D) -> Result<Slug, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    Ok(Slug(s))
  }
}
