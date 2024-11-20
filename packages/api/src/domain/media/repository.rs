use std::future::Future;

use crate::shared::vo::UUID4;

use super::Media;

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
}

pub trait MediaRepository {
  /**
   * Cria uma nova media.
   */
  fn create(&mut self, media: &Media) -> impl Future<Output = Result<(), Error>>;

  /**
   * Atualiza uma media.
   */
  fn update(&mut self, media: &Media) -> impl Future<Output = Result<(), Error>>;

  /**
   * Busca uma media pelo seu identificador e retorna a media encontrada ou None caso não exista.
   */
  fn find_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<Option<Media>, Error>>;

  /**
   * Deleta uma media pelo seu identificador.
   */
  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), Error>>;
}
