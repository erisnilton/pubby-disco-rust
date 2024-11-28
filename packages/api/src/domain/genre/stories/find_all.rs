use crate::{
  domain::genre::FindAllQuery,
  shared::{paged::RequestPageParams, vo::UUID4},
};

pub enum Error {
  RepositoryError(crate::domain::genre::repository::Error),
}
pub struct Input {
  pub page: RequestPageParams,
  pub parent_id: Option<UUID4>,
  pub search: Option<String>,
}
pub async fn execute(
  genre_repository: &mut impl crate::domain::genre::GenreRepository,
  input: Input,
) -> Result<crate::shared::paged::Paged<crate::domain::genre::genre_entity::Genre>, Error> {
  let result = genre_repository
    .find_all(&FindAllQuery {
      page: input.page,
      parent_id: input.parent_id,
      search: input.search,
    })
    .await
    .map_err(Error::RepositoryError)?;

  Ok(result)
}
