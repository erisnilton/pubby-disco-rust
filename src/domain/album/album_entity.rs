use std::collections::HashSet;

use serde_json::json;

use crate::shared::vo::{GetChanges, IntoRecord, UUID4};

use super::dto::UpdateAlbumDto;

pub enum AlbumType {
  Single,
  EP,
  Album,
  Compilation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlbumEntity {
  pub id: UUID4,
  pub name: String,
  pub cover: Option<String>,
  pub release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<i8>,
  pub artist_ids: HashSet<UUID4>,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
}

impl AlbumEntity {
  pub fn apply_changes(&mut self, old_value: &UpdateAlbumDto, new_value: &UpdateAlbumDto) {
    if old_value.name != new_value.name {
      self.name = new_value.name.clone().unwrap_or_default();
    }
    if old_value.cover != new_value.cover {
      self.cover = new_value.cover.clone();
    }
    if old_value.release_date != new_value.release_date {
      self.release_date = new_value.release_date;
    }
    if old_value.parental_rating != new_value.parental_rating {
      self.parental_rating = new_value.parental_rating;
    }

    if old_value.artist_ids != new_value.artist_ids {
      self.artist_ids = new_value.artist_ids.clone().unwrap_or_default();
    }
  }
}

impl Default for AlbumEntity {
  fn default() -> Self {
    Self {
      id: UUID4::generate(),
      name: String::new(),
      cover: Some(String::new()),
      release_date: None,
      parental_rating: None,
      artist_ids: HashSet::new(),
      created_at: chrono::Utc::now().naive_utc(),
      updated_at: chrono::Utc::now().naive_utc(),
    }
  }
}

impl IntoRecord for AlbumEntity {
  fn into(&self) -> serde_json::Value {
    json!({
      "id": self.id.to_string(),
      "name": self.name,
      "cover": self.cover,
      "release_date": self.release_date,
      "parental_rating": self.parental_rating,
      "artist_ids": self.artist_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
      "created_at": self.created_at,
      "updated_at": self.updated_at,
    })
  }
}

impl From<serde_json::Value> for AlbumEntity {
  fn from(value: serde_json::Value) -> Self {
    Self {
      id: UUID4::new(value["id"].as_str().unwrap_or_default()).unwrap(),
      name: value["name"].as_str().unwrap_or_default().to_string(),
      cover: value["cover"].as_str().map(|s| s.to_string()),
      release_date: value["release_date"]
        .as_str()
        .map(|date| chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap_or_default()),
      parental_rating: value["parental_rating"].as_i64().map(|i| i as i8),
      artist_ids: value["artist_ids"]
        .as_array()
        .map(|ids| {
          ids
            .iter()
            .map(|id| UUID4::new(id.as_str().unwrap_or_default()).unwrap_or_default())
            .collect::<HashSet<UUID4>>()
        })
        .unwrap_or_default(),
      created_at: chrono::NaiveDateTime::parse_from_str(
        value["created_at"].as_str().unwrap_or_default(),
        "%Y-%m-%dT%H:%M:%S%.f",
      )
      .unwrap_or_default(),
      updated_at: chrono::NaiveDateTime::parse_from_str(
        value["updated_at"].as_str().unwrap_or_default(),
        "%Y-%m-%dT%H:%M:%S%.f",
      )
      .unwrap_or_default(),
    }
  }
}

impl GetChanges<UpdateAlbumDto> for AlbumEntity {
  fn get_changes(&self, changes: UpdateAlbumDto) -> (UpdateAlbumDto, UpdateAlbumDto) {
    let mut old_value = UpdateAlbumDto::default();
    let mut new_value = UpdateAlbumDto::default();

    if changes.name.is_some() && changes.name != Some(self.name.clone()) {
      old_value.name = Some(self.name.clone());
      new_value.name = changes.name;
    }

    if changes.cover.is_some() && changes.cover != self.cover {
      old_value.cover = self.cover.clone();
      new_value.cover = changes.cover;
    }

    if changes.release_date.is_some() && changes.release_date != self.release_date {
      old_value.release_date = self.release_date;
      new_value.release_date = changes.release_date;
    }

    if changes.parental_rating.is_some() && changes.parental_rating != self.parental_rating {
      old_value.parental_rating = self.parental_rating;
      new_value.parental_rating = changes.parental_rating;
    }

    (old_value, new_value)
  }
}
