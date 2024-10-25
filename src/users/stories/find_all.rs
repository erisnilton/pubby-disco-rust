use crate::users::{
    dto::{PageParams, Paged, UserPresenterDTO},
    repository::{UserRepository, UserRepositoryError},
};

pub async fn find_all(
    repository: &impl UserRepository,
    page_params: PageParams,
) -> Result<Paged<UserPresenterDTO>, UserRepositoryError> {
    repository.find_all(page_params).await
}
