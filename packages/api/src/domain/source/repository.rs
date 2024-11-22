use std::future::Future;

use crate::shared::vo::UUID4;

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
}

pub trait SourceRepository {
  /**
   * Cria uma nova fonte.
   */
  fn create(
    &mut self,
    source: &super::source_entity::Source,
  ) -> impl Future<Output = Result<(), Error>>;

  /**
   * Busca uma fonte pelo seu identificador e retorna a fonte encontrada ou None caso não exista.
   */
  fn find_by_id(
    &mut self,
    id: &UUID4,
  ) -> impl Future<Output = Result<Option<super::source_entity::Source>, Error>>;

  /**
   * Deleta uma fonte pelo seu identificador.
   */
  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), Error>>;
}
