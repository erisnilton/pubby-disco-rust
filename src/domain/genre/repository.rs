use std::future::Future;

use crate::shared::vo::UUID4;

use super::Genre;

#[derive(Debug)]
pub enum GenreRepositoryError {
  InternalServerError(String),
}

pub trait GenreRepository {
  fn find_by_id(&mut self, id: &UUID4)
    -> impl Future<Output = Result<Genre, GenreRepositoryError>>;
}
