use std::future::Future;

use domain::genre::{aggregate::GenreAggregate, entity::Genre};
use shared::{
  paged::{Paged, RequestPageParams},
  vo::{Slug, UUID4},
};

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
}

#[derive(Debug, Clone)]
pub struct FindAllQuery {
  pub page: RequestPageParams,
  pub parent_id: Option<UUID4>,
  pub search: Option<String>,
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

  /**
   * Busca um gênero musical pelo seu identificador e retorna seus subgêneros.
   */
  fn find_genre_and_subgenre_by_slug(
    &mut self,
    slug: &Slug,
  ) -> impl Future<Output = Result<Option<GenreAggregate>, Error>>;

  fn find_all(&mut self, query: &FindAllQuery)
    -> impl Future<Output = Result<Paged<Genre>, Error>>;
}
