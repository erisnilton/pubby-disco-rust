use super::dto::{CreateUserDto, UserPresenterDTO};

pub enum UserRepositoryError {
    Conflict(String),
    InternalServerError(String),
}

pub trait UserRepository {
    async fn find_all(&self) -> Result<Vec<UserPresenterDTO>, UserRepositoryError>;
    async fn create(&self, user: CreateUserDto) -> Result<UserPresenterDTO, UserRepositoryError>;
}
