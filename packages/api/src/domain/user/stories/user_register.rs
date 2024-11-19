use validator::{Validate, ValidationErrors};

use crate::{
  domain::user::{User, UserRepository, UserRepositoryError},
  shared::password_hash::PasswordHash,
};

#[derive(Debug)]
pub enum CreateUserStoryError {
  UserAlreadyExists,
  InvalidInput(ValidationErrors),
  RepositoryError(UserRepositoryError),
}

impl From<UserRepositoryError> for CreateUserStoryError {
  fn from(error: UserRepositoryError) -> Self {
    CreateUserStoryError::RepositoryError(error)
  }
}

impl From<ValidationErrors> for CreateUserStoryError {
  fn from(error: ValidationErrors) -> Self {
    CreateUserStoryError::InvalidInput(error)
  }
}

#[derive(Debug, Clone, validator::Validate)]
pub struct Input {
  #[validate(length(min = 1, max = 80))]
  pub username: String,

  #[validate(length(min = 1, max = 128))]
  pub display_name: Option<String>,

  #[validate(length(min = 6, max = 255))]
  pub password: String,

  #[validate(email)]
  pub email: String,
}

pub async fn execute(
  user_repository: &mut impl UserRepository,
  password_hash: &impl PasswordHash,
  input: Input,
) -> Result<User, CreateUserStoryError> {
  input.validate()?;

  let user_exists = user_repository
    .find_by_username(input.username.clone())
    .await?
    .is_some();

  if user_exists {
    return Err(CreateUserStoryError::UserAlreadyExists);
  }
  let user = User::builder()
    .username(input.username.clone())
    .password(password_hash.hash_password(&input.password))
    .email(input.email.clone())
    .display_name(input.display_name.clone().unwrap_or_default())
    .build();

  user_repository.create(&user).await?;

  Ok(user)
}

#[cfg(test)]
mod test {
  use crate::infra::bcrypt::BcryptPasswordHash;

  use super::*;

  #[tokio::test]
  async fn create_user_should_return_error_when_user_exists() {
    let mut user_repository = crate::infra::in_memory::InMemoryUserRepository::default();
    let password_hash = BcryptPasswordHash;

    let input = Input {
      username: String::from("username"),
      password: String::from("password"),
      email: String::from("user@gmail.com"),
      display_name: None,
    };

    let user = User::builder()
      .username(input.username.clone())
      .password(password_hash.hash_password(&input.password))
      .email(input.email.clone())
      .display_name(input.display_name.clone().unwrap_or_default())
      .build();

    user_repository.create(&user).await.unwrap();

    let result = crate::domain::user::stories::user_register::execute(
      &mut user_repository,
      &password_hash,
      input,
    )
    .await;

    assert!(
      matches!(result, Err(CreateUserStoryError::UserAlreadyExists)),
      "Não foi retornado o erro de usuário já existente"
    );
  }

  #[tokio::test]
  async fn create_user_should_return_user_when_user_is_created() {
    let mut user_repository = crate::infra::in_memory::InMemoryUserRepository::default();
    let password_hash = BcryptPasswordHash;

    let input = Input {
      username: String::from("username"),
      password: String::from("password"),
      email: String::from("user@gmail.com"),
      display_name: None,
    };

    let result = crate::domain::user::stories::user_register::execute(
      &mut user_repository,
      &password_hash,
      input.clone(),
    )
    .await
    .expect("Erro ao cadastrar usuário");

    assert_eq!(result.username(), &input.username);
  }
}
