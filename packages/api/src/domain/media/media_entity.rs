use domain_proc_macros::Entity;

use crate::shared::{util::naive_now, vo::Slug};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MediaType {
  Song,
  Instrumental,
}

#[derive(Debug, Entity, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Media {
  id: crate::shared::vo::UUID4,
  name: String,
  media_type: MediaType,
  slug: Slug,
  release_date: Option<chrono::NaiveDate>,
  cover: Option<String>,
  parental_rating: u8,
  genre_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  composer_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  interpreter_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  album_ids: std::collections::HashSet<crate::shared::vo::UUID4>,
  is_single: bool,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
}

impl Media {
  pub fn apply_changes(&mut self, changes: &crate::domain::media::contribution::changes::Changes) {
    if let Some(value) = &changes.name {
      self.name = value.clone();
    }
    if let Some(value) = &changes.media_type {
      self.media_type = value.clone();
    }
    if let Some(value) = &changes.slug {
      self.slug = value.clone();
    }
    if let Some(value) = changes.release_date {
      self.release_date = Some(value);
    }
    if let Some(value) = &changes.cover {
      self.cover = value.clone();
    }
    if let Some(value) = &changes.genre_ids {
      self.genre_ids = value.clone();
    }
    if let Some(value) = changes.parental_rating {
      self.parental_rating = value;
    }
    if let Some(value) = &changes.composer_ids {
      self.composer_ids = value.clone();
    }
    if let Some(value) = &changes.interpreter_ids {
      self.interpreter_ids = value.clone();
    }
    if let Some(value) = &changes.album_ids {
      self.album_ids = value.clone();
    }

    self.updated_at = naive_now();
  }
}

impl Default for Media {
  fn default() -> Self {
    Media {
      id: crate::shared::vo::UUID4::default(),
      name: String::default(),
      media_type: MediaType::Song,
      release_date: None,
      slug: Slug::default(),
      cover: None,
      parental_rating: 0,
      genre_ids: std::collections::HashSet::default(),
      composer_ids: std::collections::HashSet::default(),
      interpreter_ids: std::collections::HashSet::default(),
      album_ids: std::collections::HashSet::default(),
      is_single: false,
      created_at: naive_now(),
      updated_at: naive_now(),
    }
  }
}
