use crate::users::{
    dto::{CreateUserDto, UserPresenterDTO},
    repository::{UserRepository, UserRepositoryError},
};

pub async fn create_user(
    repository: &impl UserRepository,
    input: CreateUserDto,
) -> Result<UserPresenterDTO, UserRepositoryError> {
    let user = repository.create(input).await?;

    Ok(UserPresenterDTO {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
    })
}
