use std::future::Future;

use crate::shared::vo::{Slug, UUID4};

use super::Artist;

#[derive(Debug, Clone)]
pub enum ArtistRepositoryError {
  Conflict(String),
  DatabaseError(String),
  NotFound,
}

pub trait ArtistRepository {
  fn create(
    &mut self,
    input: &Artist,
  ) -> impl Future<Output = Result<Artist, ArtistRepositoryError>>;
  fn update(
    &mut self,
    input: &Artist,
  ) -> impl Future<Output = Result<Artist, ArtistRepositoryError>>;
  fn find_by_id(
    &mut self,
    id: &UUID4,
  ) -> impl Future<Output = Result<Option<Artist>, ArtistRepositoryError>>;
  fn delete_by_id(&mut self, id: &UUID4)
    -> impl Future<Output = Result<(), ArtistRepositoryError>>;

  fn find_by_slug(
    &mut self,
    slug: &Slug,
  ) -> impl Future<Output = Result<Artist, ArtistRepositoryError>>;
}
