use crate::domain::artists::{repository::{ArtistRepository, ArtistRepositoryError}, Artist};

pub async fn find_artist_by_slug(
    repository: &impl ArtistRepository,
    slug: &str,
) -> Result<Artist, ArtistRepositoryError> {
    let artist = repository.find_by_slug(slug).await?;

    Ok(artist)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::infra::artist_repository::InMemoryArtistRepository;

    #[tokio::test]
    async fn test_find_artist_by_slug() {
        let artist = Artist::new("name".to_string(), "slug".to_string(), "country".to_string());
        let mut repository = InMemoryArtistRepository::default();

        repository.artists.insert(artist.slug.clone(), artist.clone());

        let result = find_artist_by_slug(&repository, &artist.slug).await.unwrap();
        assert_eq!(result.id, artist.id);
    }

    #[tokio::test]
    async fn test_find_artist_by_slug_not_found() {
        let repository = InMemoryArtistRepository::default();

        let result = find_artist_by_slug(&repository, "not_found").await;
        assert!(result.is_err());
    }
}