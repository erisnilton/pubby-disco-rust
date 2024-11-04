use std::future::Future;

use crate::shared::vo::UUID4;

use super::Genre;

#[derive(Debug, Clone)]
pub enum GenreRepositoryError {
  DatabaseError(String),
}

pub trait GenreRepository {
  fn create(&mut self, genre: Genre) -> impl Future<Output = Result<Genre, GenreRepositoryError>>;
  fn update(&mut self, genre: Genre) -> impl Future<Output = Result<Genre, GenreRepositoryError>>;
  fn find_by_id(
    &mut self,
    id: &UUID4,
  ) -> impl Future<Output = Result<Option<Genre>, GenreRepositoryError>>;

  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), GenreRepositoryError>>;
}
