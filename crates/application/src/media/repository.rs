use std::collections::HashSet;
use std::future::Future;

use domain::media::{
  aggregate::MediaAggregate,
  entity::{Media, MediaType},
};
use shared::{
  paged::{Paged, RequestPageParams},
  vo::{Slug, UUID4},
};

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
}

#[derive(Debug, Clone)]
pub struct FindByQuery {
  pub page: RequestPageParams,
  pub search: Option<String>,
  pub release_date: Option<chrono::NaiveDate>,
  pub min_release_date: Option<chrono::NaiveDate>,
  pub max_release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub min_parental_rating: Option<u8>,
  pub max_parental_rating: Option<u8>,
  pub is_single: Option<bool>,
  pub media_type: Option<MediaType>,
  pub slug: Option<Slug>,
  pub artist_ids: Option<HashSet<UUID4>>,
  pub composer_ids: Option<HashSet<UUID4>>,
  pub genre_ids: Option<HashSet<UUID4>>,
  pub album_ids: Option<HashSet<UUID4>>,
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

  fn find_by(
    &mut self,
    query: &FindByQuery,
  ) -> impl Future<Output = Result<Paged<MediaAggregate>, Error>>;
}
