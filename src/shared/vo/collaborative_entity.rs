use core::panic;

use serde::{Deserialize, Serialize};

use crate::domain::{
  album::{dto::UpdateAlbumDto, AlbumEntity},
  artists::{dto::UpdateArtistDto, Artist},
  genre::{dto::UpdateGenreDto, Genre},
};

use super::{collaborative::CollaborativeEntityName, UUID4};

#[derive(Debug, Clone)]
pub enum CollaborativeEntity {
  Genre(Genre),
  Artist(Artist),
  Album(AlbumEntity),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UpdateCollaborativeEntity {
  Genre(UpdateGenreDto),
  Artist(UpdateArtistDto),
  Album(UpdateAlbumDto),
}

impl CollaborativeEntity {
  pub fn id(&self) -> String {
    match self {
      CollaborativeEntity::Genre(genre) => genre.id.0.clone(),
      CollaborativeEntity::Artist(artist) => artist.id.0.clone(),
      CollaborativeEntity::Album(album) => album.id.0.clone(),
    }
  }
  pub fn name(&self) -> CollaborativeEntityName {
    match self {
      CollaborativeEntity::Genre(..) => CollaborativeEntityName::Genre,
      CollaborativeEntity::Artist(..) => CollaborativeEntityName::Artist,
      CollaborativeEntity::Album(..) => CollaborativeEntityName::Album,
    }
  }
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub enum CollaborativeEntityId {
  Genre(UUID4),
  Artist(UUID4),
  Album(UUID4),
}

pub trait GetChanges<T> {
  fn get_changes(&self, changes: T) -> (T, T);
}

pub trait IntoRecord {
  fn into(&self) -> serde_json::Value;
}

impl IntoRecord for CollaborativeEntity {
  fn into(&self) -> serde_json::Value {
    match self {
      CollaborativeEntity::Genre(genre) => IntoRecord::into(genre),
      CollaborativeEntity::Artist(artist) => IntoRecord::into(artist),
      CollaborativeEntity::Album(album) => IntoRecord::into(album),
    }
  }
}

impl IntoRecord for UpdateCollaborativeEntity {
  fn into(&self) -> serde_json::Value {
    match self {
      UpdateCollaborativeEntity::Genre(genre_dto) => IntoRecord::into(genre_dto),
      UpdateCollaborativeEntity::Artist(artist_dto) => IntoRecord::into(artist_dto),
      UpdateCollaborativeEntity::Album(album_dto) => IntoRecord::into(album_dto),
    }
  }
}

impl GetChanges<UpdateCollaborativeEntity> for CollaborativeEntity {
  fn get_changes(
    &self,
    changes: UpdateCollaborativeEntity,
  ) -> (UpdateCollaborativeEntity, UpdateCollaborativeEntity) {
    match self {
      CollaborativeEntity::Genre(genre) => match changes {
        UpdateCollaborativeEntity::Genre(genre_changes) => {
          let (old_value, new_value) = genre.get_changes(genre_changes);

          return (
            UpdateCollaborativeEntity::Genre(old_value),
            UpdateCollaborativeEntity::Genre(new_value),
          );
        }
        value => panic!(
          "Era esperado um UpdateCollaborativeEntityDto::Genre, mas foi recebido: {:#?}",
          value
        ),
      },
      CollaborativeEntity::Album(album) => match changes {
        UpdateCollaborativeEntity::Album(album_changes) => {
          let (old_value, new_value) = album.get_changes(album_changes);

          return (
            UpdateCollaborativeEntity::Album(old_value),
            UpdateCollaborativeEntity::Album(new_value),
          );
        }
        value => panic!(
          "Era esperado um UpdateCollaborativeEntityDto::Album, mas foi recebido: {:#?}",
          value
        ),
      },
      CollaborativeEntity::Artist(artist) => match changes {
        UpdateCollaborativeEntity::Artist(artist_changes) => {
          let (old_value, new_value) = artist.get_changes(artist_changes);

          return (
            UpdateCollaborativeEntity::Artist(old_value),
            UpdateCollaborativeEntity::Artist(new_value),
          );
        }
        value => panic!(
          "Era esperado um UpdateCollaborativeEntityDto::Artist, mas foi recebido: {:#?}",
          value
        ),
      },
    }
  }
}
