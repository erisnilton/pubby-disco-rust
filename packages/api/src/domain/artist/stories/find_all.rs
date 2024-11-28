use crate::shared::paged::RequestPageParams;

pub enum Error {
  RepositoryError(crate::domain::artist::repository::Error),
}
pub struct Input {
  pub page: RequestPageParams,
  pub search: Option<String>,
  pub country: Option<String>,
}
pub async fn execute(
  artist_repository: &mut impl crate::domain::artist::repository::ArtistRepository,
  input: Input,
) -> Result<crate::shared::paged::Paged<crate::domain::artist::artist_entity::Artist>, Error> {
  let result = artist_repository
    .find_all(&crate::domain::artist::repository::FindAllQuery {
      page: input.page,
      search: input.search,
      country: input.country,
    })
    .await
    .map_err(Error::RepositoryError)?;

  Ok(result)
}
