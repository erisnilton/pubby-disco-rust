use std::future::Future;

use crate::shared::vo::UUID4;

use super::AlbumEntity;

#[derive(Debug, Clone)]
pub enum AlbumRepositoryError {
  DatabaseError(String),
}

pub trait AlbumRepository {
  fn create(
    &mut self,
    album: AlbumEntity,
  ) -> impl Future<Output = Result<AlbumEntity, AlbumRepositoryError>>;
  fn update(
    &mut self,
    album: AlbumEntity,
  ) -> impl Future<Output = Result<AlbumEntity, AlbumRepositoryError>>;
  fn find_by_id(
    &mut self,
    id: &UUID4,
  ) -> impl Future<Output = Result<Option<AlbumEntity>, AlbumRepositoryError>>;

  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), AlbumRepositoryError>>;
}
