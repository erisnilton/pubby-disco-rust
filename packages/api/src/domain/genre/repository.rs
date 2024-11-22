use std::future::Future;

use crate::shared::vo::UUID4;

use super::Genre;

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
}

pub trait GenreRepository {
  /**
   * Cria um novo gênero musical.
   */
  fn create(&mut self, genre: &Genre) -> impl Future<Output = Result<(), Error>>;

  /**
   * Atualiza um gênero musical.
   */
  fn update(&mut self, genre: &Genre) -> impl Future<Output = Result<(), Error>>;

  /**
   * Busca um gênero musical pelo seu identificador e retorna o gênero musical encontrado ou None caso não exista.
   */
  fn find_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<Option<Genre>, Error>>;

  /**
   * Deleta um gênero musical pelo seu identificador.
   */
  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), Error>>;
}
