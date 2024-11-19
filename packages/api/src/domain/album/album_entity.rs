use std::{fmt::Display, str::FromStr};

use crate::shared::util::naive_now;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlbumType {
  Single,
  EP,
  Album,
  Compilation,
}

impl Display for AlbumType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Single => write!(f, "Single"),
      Self::EP => write!(f, "EP"),
      Self::Album => write!(f, "Album"),
      Self::Compilation => write!(f, "Compilation"),
    }
  }
}

impl FromStr for AlbumType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Single" => Ok(Self::Single),
      "EP" => Ok(Self::EP),
      "Album" => Ok(Self::Album),
      "Compilation" => Ok(Self::Compilation),
      _ => Err(format!("Invalid AlbumType: {}", s)),
    }
  }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Album {
  pub id: crate::shared::vo::UUID4,

  pub name: String,
  pub album_type: AlbumType,
  pub cover: Option<String>,
  pub release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub artist_ids: std::collections::HashSet<crate::shared::vo::UUID4>,

  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl Album {
  pub fn apply_changes(&mut self, changes: &super::contribution::changes::Changes) {
    if let Some(value) = &changes.name {
      self.name = value.clone();
    }

    if let Some(value) = &changes.album_type {
      self.album_type = value.clone();
    }

    if let Some(value) = &changes.cover {
      self.cover = Some(value.clone());
    }

    if let Some(value) = changes.release_date {
      self.release_date = Some(value);
    }

    if let Some(value) = changes.parental_rating {
      self.parental_rating = Some(value);
    }

    if let Some(value) = &changes.artist_ids {
      self.artist_ids = value.clone();
    }

    self.updated_at = naive_now();
  }
}

impl Default for Album {
  fn default() -> Self {
    let now = naive_now();

    Self {
      id: crate::shared::vo::UUID4::generate(),
      name: String::new(),
      album_type: AlbumType::Album,
      cover: Some(String::new()),
      release_date: None,
      parental_rating: None,
      artist_ids: std::collections::HashSet::new(),
      created_at: now,
      updated_at: now,
    }
  }
}
