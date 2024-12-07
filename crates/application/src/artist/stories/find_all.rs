use domain::artist::entity::Artist;
use shared::paged::{Paged, RequestPageParams};

use crate::artist::repository::FindAllQuery;

pub enum Error {
  RepositoryError(crate::artist::repository::Error),
}
pub struct Input {
  pub page: RequestPageParams,
  pub search: Option<String>,
  pub country: Option<String>,
}
pub async fn execute(
  artist_repository: &mut impl crate::artist::repository::ArtistRepository,
  input: Input,
) -> Result<Paged<Artist>, Error> {
  let result = artist_repository
    .find_all(&FindAllQuery {
      page: input.page,
      search: input.search,
      country: input.country,
    })
    .await
    .map_err(Error::RepositoryError)?;

  Ok(result)
}
