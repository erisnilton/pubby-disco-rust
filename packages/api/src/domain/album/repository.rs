use std::future::Future;

use crate::shared::vo::{Slug, UUID4};

use super::{album_aggregate::AlbumAggregate, Album};

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
}

#[derive(Debug, Clone)]
pub struct FindAllQuery {
  pub page: crate::shared::paged::RequestPageParams,
  pub name: Option<String>,
  pub slug: Option<Slug>,
  pub artist_ids: Option<Vec<UUID4>>,
  pub album_type: Option<super::AlbumType>,
  pub release_date: Option<chrono::NaiveDate>,
  pub min_release_date: Option<chrono::NaiveDate>,
  pub max_release_date: Option<chrono::NaiveDate>,
  pub parental_rating: Option<u8>,
  pub min_parental_rating: Option<u8>,
  pub max_parental_rating: Option<u8>,
  pub search: Option<String>,
}

pub trait AlbumRepository {
  /**
   * Cria um novo álbum.
   */
  fn create(&mut self, album: &Album) -> impl Future<Output = Result<Album, Error>>;

  /**
   * Atualiza um álbum.
   */
  fn update(&mut self, album: &Album) -> impl Future<Output = Result<Album, Error>>;

  /**
   * Busca um álbum pelo seu identificador e retorna o álbum encontrado ou None caso não exista.
   */
  fn find_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<Option<Album>, Error>>;

  /**
   * Deleta um álbum pelo seu identificador.
   */
  fn delete_by_id(&mut self, id: &UUID4) -> impl Future<Output = Result<(), Error>>;

  /**
   * Busca um álbum pelo seu slug e retorna o álbum encontrado ou None caso não exista.
   */
  fn find_by_slug(
    &mut self,
    slug: &Slug,
    artist_slug: &Slug,
  ) -> impl Future<Output = Result<Option<super::album_aggregate::AlbumAggregate>, Error>>;

  fn find_by(
    &mut self,
    query: &FindAllQuery,
  ) -> impl Future<Output = Result<crate::shared::paged::Paged<AlbumAggregate>, Error>>;
}
