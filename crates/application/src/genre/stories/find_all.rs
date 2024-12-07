use domain::genre::entity::Genre;
use shared::{
  paged::{Paged, RequestPageParams},
  vo::UUID4,
};

use crate::genre::repository::FindAllQuery;

pub enum Error {
  RepositoryError(crate::genre::repository::Error),
}
pub struct Input {
  pub page: RequestPageParams,
  pub parent_id: Option<UUID4>,
  pub search: Option<String>,
}
pub async fn execute(
  genre_repository: &mut impl crate::genre::repository::GenreRepository,
  input: Input,
) -> Result<Paged<Genre>, Error> {
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
