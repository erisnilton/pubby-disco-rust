use crate::*;

#[derive(Debug, Default)]
pub struct InMemoryAlbumRepository {
  pub albums: HashMap<String, domain::album::Album>,
}

impl InMemoryAlbumRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      albums: HashMap::new(),
    }
  }
}

impl domain::album::repository::AlbumRepository for InMemoryAlbumRepository {
  async fn create(
    &mut self,
    input: &domain::album::Album,
  ) -> Result<domain::album::Album, crate::domain::album::repository::Error> {
    self.albums.insert(input.id().to_string(), input.clone());
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<domain::album::Album>, crate::domain::album::repository::Error> {
    Ok(self.albums.get(&id.0).cloned())
  }

  async fn update(
    &mut self,
    album: &domain::album::Album,
  ) -> Result<domain::album::Album, crate::domain::album::repository::Error> {
    self.albums.insert(album.id().to_string(), album.clone());
    Ok(album.clone())
  }
  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::album::repository::Error> {
    self.albums.remove(&id.0);
    Ok(())
  }

  async fn find_by_slug(
    &mut self,
    _slug: &shared::vo::Slug,
    _artist_slug: &shared::vo::Slug,
  ) -> Result<
    Option<domain::album::album_aggregate::AlbumAggregate>,
    domain::album::repository::Error,
  > {
    todo!()
  }

  async fn find_by(
    &mut self,
    _query: &domain::album::repository::FindAllQuery,
  ) -> Result<
    crate::shared::paged::Paged<domain::album::album_aggregate::AlbumAggregate>,
    domain::album::repository::Error,
  > {
    todo!()
  }
}
