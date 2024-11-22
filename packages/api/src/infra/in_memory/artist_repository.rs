use std::collections::HashMap;

use crate::*;

#[derive(Debug, Default)]
pub struct InMemoryArtistRepository {
  pub artists: HashMap<String, domain::artist::Artist>,
}

impl InMemoryArtistRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      artists: HashMap::new(),
    }
  }
}

impl domain::artist::repository::ArtistRepository for InMemoryArtistRepository {
  async fn find_by_slug(
    &mut self,
    slug: &shared::vo::Slug,
  ) -> Result<Option<domain::artist::Artist>, domain::artist::repository::Error> {
    let artist = self
      .artists
      .values()
      .find(|artist| artist.slug() == slug)
      .cloned();
    Ok(artist)
  }

  async fn create(
    &mut self,
    input: &domain::artist::Artist,
  ) -> Result<domain::artist::Artist, domain::artist::repository::Error> {
    self.artists.insert(input.id().to_string(), input.clone());
    Ok(input.clone())
  }

  async fn update(
    &mut self,
    input: &domain::artist::Artist,
  ) -> Result<domain::artist::Artist, domain::artist::repository::Error> {
    self.artists.insert(input.id().to_string(), input.clone());
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &shared::vo::UUID4,
  ) -> Result<Option<domain::artist::Artist>, domain::artist::repository::Error> {
    let artist = self.artists.get(&id.to_string()).cloned();
    Ok(artist)
  }

  async fn delete_by_id(
    &mut self,
    id: &shared::vo::UUID4,
  ) -> Result<(), domain::artist::repository::Error> {
    self.artists.remove(&id.to_string());
    Ok(())
  }
}

#[cfg(test)]
pub mod tests {
  use domain::artist::repository::ArtistRepository;

  use super::*;

  #[tokio::test]
  async fn test_find_by_slug() {
    let artist = domain::artist::Artist::builder()
      .name(String::from("name"))
      .slug(shared::vo::Slug::new("slug").unwrap())
      .country(Some(String::from("BR")))
      .build();

    let mut artists = HashMap::new();

    artists.insert(artist.id().to_string(), artist.clone());

    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.find_by_slug(artist.slug()).await.unwrap();

    assert_eq!(result, Some(artist));
  }

  #[tokio::test]
  async fn test_find_by_slug_not_found() {
    let artist = domain::artist::Artist::builder()
      .name(String::from("name"))
      .slug(shared::vo::Slug::new("slug").unwrap())
      .country(Some(String::from("BR")))
      .build();

    let artists = HashMap::new();
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo
      .find_by_slug(artist.slug())
      .await
      .expect("Error finding artist");

    assert_eq!(result, None);
  }

  #[tokio::test]
  async fn test_create() {
    let artist = domain::artist::Artist::builder()
      .name(String::from("name"))
      .slug(shared::vo::Slug::new("slug").unwrap())
      .country(Some(String::from("BR")))
      .build();

    let artists = HashMap::new();
    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.create(&artist).await.expect("Error creating artist");
    assert_eq!(result.id(), artist.id());
  }

  #[tokio::test]
  async fn test_update() {
    let artist = domain::artist::Artist::builder()
      .name(String::from("name"))
      .slug(shared::vo::Slug::new("slug").unwrap())
      .country(Some(String::from("BR")))
      .build();

    let mut artists = HashMap::new();

    artists.insert(artist.id().to_string(), artist.clone());

    let mut repo = InMemoryArtistRepository { artists };

    let result = repo.update(&artist).await.expect("Error updating artist");

    assert_eq!(result, artist);
  }

  #[tokio::test]
  async fn test_find_by_id() {
    let artist = domain::artist::Artist::builder()
      .name(String::from("name"))
      .slug(shared::vo::Slug::new("slug").unwrap())
      .country(Some(String::from("BR")))
      .build();

    let mut artists = HashMap::new();

    artists.insert(artist.id().to_string(), artist.clone());

    let mut repo = InMemoryArtistRepository { artists };

    let result = repo
      .find_by_id(artist.id())
      .await
      .expect("Error finding artist");

    assert_eq!(result, Some(artist));
  }
}
