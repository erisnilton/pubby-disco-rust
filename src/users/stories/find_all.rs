use crate::users::{
    dto::UserPresenterDTO,
    repository::{UserRepository, UserRepositoryError},
};

pub async fn find_all(
    repository: &impl UserRepository,
) -> Result<Vec<UserPresenterDTO>, UserRepositoryError> {
    repository.find_all().await
}
