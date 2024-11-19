use std::{fmt::Display, str::FromStr};

use domain_proc_macros::Entity;

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

#[derive(Entity, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Album {
  id: crate::shared::vo::UUID4,

  name: String,
  album_type: AlbumType,
  cover: Option<String>,
  release_date: Option<chrono::NaiveDate>,
  parental_rating: Option<u8>,
  artist_ids: std::collections::HashSet<crate::shared::vo::UUID4>,

  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
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
