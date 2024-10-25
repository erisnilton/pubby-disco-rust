use std::future::Future;

use super::Artist;

#[derive(Debug)]
pub enum ArtistRepositoryError {
    Conflict(String),
    InternalServerError(String),
    NotFound,
}

pub trait ArtistRepository {
    // fn find_all(
    //     &self,
    //     page_params: PageParams,
    // ) -> impl Future<Output = Result<Paged<Artist>, ArtistRepositoryError>>;
    fn find_by_slug(
        &self,
        slug: &str,
    ) -> impl Future<Output = Result<Artist, ArtistRepositoryError>>;
    fn create(&self, input: &Artist)
        -> impl Future<Output = Result<Artist, ArtistRepositoryError>>;
}
