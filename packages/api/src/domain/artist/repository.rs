use std::future::Future;

use crate::shared::vo::{Slug, UUID4};

use super::Artist;

#[derive(Debug, Clone)]
pub enum Error {
  Conflict(String),
  DatabaseError(String),
}

pub trait ArtistRepository {
  /**
   * Cria um novo artista.
   */
  fn create(&mut self, input: &Artist) -> impl Future<Output = Result<Artist, Error>>;

  /**
   * Atualiza um artista.
   */
  fn update(&mut self, input: &Artist) -> impl Future<Output = Result<Artist, Error>>;

  /**
   * Busca um artista pelo seu identificador e retorna o artista encontrado ou None caso não exista.
   */
  fn find_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<Option<Artist>, Error>>;

  /**
   * Deleta um artista pelo seu identificador.
   */
  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), Error>>;

  /**
   * Busca um artista pelo seu slug e retorna o artista encontrado ou None caso não exista.
   */
  fn find_by_slug(&mut self, slug: &Slug) -> impl Future<Output = Result<Option<Artist>, Error>>;
}
