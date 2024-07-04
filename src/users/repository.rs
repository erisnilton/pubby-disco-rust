use super::dto::{CreateUserDto, PageParams, Paged, UserPresenterDTO};

#[derive(Debug)]
pub enum UserRepositoryError {
    Conflict(String),
    InternalServerError(String),
}

impl std::fmt::Display for UserRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(message) => write!(f, "Conflict: {}", message),
            Self::InternalServerError(message) => write!(f, "Internal Server Error: {}", message),
        }
    }
}

pub trait UserRepository {
    async fn find_all(
        &self,
        page_params: PageParams,
    ) -> Result<Paged<UserPresenterDTO>, UserRepositoryError>;
    async fn create(&self, user: CreateUserDto) -> Result<UserPresenterDTO, UserRepositoryError>;
}
