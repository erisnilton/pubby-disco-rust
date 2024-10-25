use std::collections::HashMap;

use crate::domain::artists::{
    repository::{ArtistRepository, ArtistRepositoryError},
    Artist,
};

#[derive(Debug, Default)]
pub struct InMemoryArtistRepository {
    pub artists: HashMap<String, Artist>,
}

impl ArtistRepository for InMemoryArtistRepository {
    async fn find_by_slug(
        &self,
        slug: &str,
    ) -> Result<Artist, crate::domain::artists::repository::ArtistRepositoryError> {
        match self.artists.get(slug) {
            Some(artist) => Ok(artist.clone()),
            None => Err(ArtistRepositoryError::NotFound),
        }
    }

    async fn create(
        &self,
        _input: &Artist,
    ) -> Result<Artist, crate::domain::artists::repository::ArtistRepositoryError> {
        todo!()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::domain::artists::Artist;

    #[tokio::test]
    async fn test_find_by_slug() {
        let artist = Artist::new(
            "name".to_string(),
            "slug".to_string(),
            "country".to_string(),
        );
        let mut artists = HashMap::new();
        artists.insert(artist.slug.clone(), artist.clone());
        let repo = InMemoryArtistRepository { artists };

        let result = repo.find_by_slug(&artist.slug).await.unwrap();
        assert_eq!(result.id, artist.id);
    }

    #[tokio::test]
    async fn test_find_by_slug_not_found() {
        let artist = Artist::new(
            "name".to_string(),
            "slug".to_string(),
            "country".to_string(),
        );
        let artists = HashMap::new();
        let repo = InMemoryArtistRepository { artists };

        let result = repo.find_by_slug(&artist.slug).await;
        assert!(result.is_err());
    }
}
