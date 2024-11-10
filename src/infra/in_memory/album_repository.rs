use std::collections::HashMap;

use crate::{
  domain::album::{AlbumEntity, AlbumRepository},
  AppState,
};

#[derive(Debug, Default)]
pub struct InMemoryAlbumRepository {
  pub albums: HashMap<String, AlbumEntity>,
}

impl InMemoryAlbumRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      albums: HashMap::new(),
    }
  }
}

impl AlbumRepository for InMemoryAlbumRepository {
  async fn create(
    &mut self,
    input: AlbumEntity,
  ) -> Result<AlbumEntity, crate::domain::album::AlbumRepositoryError> {
    self.albums.insert(input.id.0.clone(), input.clone());
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<AlbumEntity>, crate::domain::album::AlbumRepositoryError> {
    Ok(self.albums.get(&id.0).cloned())
  }

  async fn update(
    &mut self,
    album: AlbumEntity,
  ) -> Result<AlbumEntity, crate::domain::album::AlbumRepositoryError> {
    self.albums.insert(album.id.0.clone(), album.clone());
    Ok(album.clone())
  }
  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::album::AlbumRepositoryError> {
    self.albums.remove(&id.0);
    Ok(())
  }
}
