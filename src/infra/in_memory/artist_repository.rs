use std::collections::HashMap;

use crate::{
  domain::artists::{
    repository::{ArtistRepository, ArtistRepositoryError},
    Artist,
  },
  shared::vo::Slug,
  AppState,
};

#[derive(Debug, Default)]
pub struct InMemoryArtistRepository {
  pub artists: HashMap<String, Artist>,
}

impl InMemoryArtistRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      artists: HashMap::new(),
    }
  }
}

impl ArtistRepository for InMemoryArtistRepository {
  async fn find_by_slug(
    &mut self,
    slug: &Slug,
  ) -> Result<Artist, crate::domain::artists::repository::ArtistRepositoryError> {
    match self.artists.get(&slug.to_string()) {
      Some(artist) => Ok(artist.clone()),
      None => Err(ArtistRepositoryError::NotFound),
    }
  }

  async fn create(
    &mut self,
    input: &Artist,
  ) -> Result<Artist, crate::domain::artists::repository::ArtistRepositoryError> {
    self.artists.insert(input.id.to_string(), input.clone());
    Ok(input.clone())
  }

  async fn update(&mut self, input: &Artist) -> Result<Artist, ArtistRepositoryError> {
    self.artists.insert(input.id.to_string(), input.clone());
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<Artist>, ArtistRepositoryError> {
    let artist = self.artists.get(&id.to_string()).cloned();
    Ok(artist)
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), ArtistRepositoryError> {
    self.artists.remove(&id.to_string());
    Ok(())
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::{domain::artists::Artist, shared::vo::Slug};

  #[tokio::test]
  async fn test_find_by_slug() {
    let artist = Artist::new(
      "name".to_string(),
      Slug::new("slug").unwrap(),
      "country".to_string(),
    );
    let mut artists = HashMap::new();
    artists.insert(artist.id.to_string(), artist.clone());
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.find_by_slug(&artist.slug).await.unwrap();
    assert_eq!(result.id, artist.id);
  }

  #[tokio::test]
  async fn test_find_by_slug_not_found() {
    let artist = Artist::new(
      "name".to_string(),
      Slug::new("slug").unwrap(),
      "country".to_string(),
    );
    let artists = HashMap::new();
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.find_by_slug(&artist.slug).await;
    assert!(result.is_err());
  }

  #[tokio::test]
  async fn test_create() {
    let artist = Artist::new(
      "name".to_string(),
      Slug::new("slug").unwrap(),
      "country".to_string(),
    );
    let artists = HashMap::new();
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.create(&artist).await.unwrap();
    assert_eq!(result.id, artist.id);
  }

  #[tokio::test]
  async fn test_update() {
    let artist = Artist::new(
      "name".to_string(),
      Slug::new("slug").unwrap(),
      "country".to_string(),
    );
    let mut artists = HashMap::new();
    artists.insert(artist.id.to_string(), artist.clone());
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.update(&artist).await.unwrap();
    assert_eq!(result.id, artist.id);
  }

  #[tokio::test]
  async fn test_find_by_id() {
    let artist = Artist::new(
      "name".to_string(),
      Slug::new("slug").unwrap(),
      "country".to_string(),
    );
    let mut artists = HashMap::new();
    artists.insert(artist.id.to_string(), artist.clone());
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.find_by_id(&artist.id).await.unwrap();
    assert_eq!(result.unwrap().id, artist.id);
  }
}
