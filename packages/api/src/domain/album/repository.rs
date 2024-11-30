use std::future::Future;

use crate::shared::vo::{Slug, UUID4};

use super::Album;

#[derive(Debug, Clone)]
pub enum Error {
  DatabaseError(String),
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
}
