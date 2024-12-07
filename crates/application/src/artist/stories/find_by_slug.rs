use domain::artist::entity::Artist;
use shared::vo::Slug;

use crate::artist::repository::ArtistRepository;

pub enum Error {
  RepositoryError(crate::artist::repository::Error),
  ArtistNotFound,
}

pub async fn execute(repository: &mut impl ArtistRepository, slug: &Slug) -> Result<Artist, Error> {
  let artist = repository
    .find_by_slug(slug)
    .await
    .map_err(Error::RepositoryError)?;

  if let Some(artist) = artist {
    return Ok(artist);
  }

  Err(Error::ArtistNotFound)
}

#[cfg(test)]
pub mod tests {
  // use super::*;
  // use crate::infra::artist_repository::InMemoryArtistRepository;

  // #[tokio::test]
  // async fn test_find_artist_by_slug() {
  //     let artist = Artist::new("name".to_string(), "slug".to_string(), "country".to_string());
  //     let mut repository = InMemoryArtistRepository::default();

  //     repository.artists.insert(artist.slug.clone(), artist.clone());

  //     let result = find_artist_by_slug(&repository, &artist.slug).await.unwrap();
  //     assert_eq!(result.id, artist.id);
  // }

  // #[tokio::test]
  // async fn test_find_artist_by_slug_not_found() {
  //     let repository = InMemoryArtistRepository::default();

  //     let result = find_artist_by_slug(&repository, "not_found").await;
  //     assert!(result.is_err());
  // }
}
