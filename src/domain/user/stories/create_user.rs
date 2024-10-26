use validator::{Validate, ValidationErrors};

use crate::domain::user::{dto::UserRegisterDto, User, UserRepository, UserRepositoryError};

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

pub async fn create_user(
  user_repository: &mut impl UserRepository,
  input: UserRegisterDto,
) -> Result<User, CreateUserStoryError> {
  input.validate()?;

  let user_exists = user_repository
    .find_by_username(input.username.clone())
    .await?
    .is_some();

  if user_exists {
    return Err(CreateUserStoryError::UserAlreadyExists);
  }

  let user = user_repository
    .create(User {
      username: input.username,
      password: input.password,
      email: input.email,
      display_name: input.display_name.unwrap_or_default(),
      ..Default::default()
    })
    .await?;

  Ok(user)
}

#[cfg(test)]
mod test {
  use std::collections::HashMap;

  use super::*;

  #[derive(Debug, Default, Clone)]
  struct InMemoryUserRepository {
    users: HashMap<String, User>,
  }

  impl UserRepository for InMemoryUserRepository {
    async fn create(&mut self, user: User) -> Result<User, UserRepositoryError> {
      self.users.insert(user.username.clone(), user.clone());
      Ok(user)
    }

    async fn find_by_username(
      &mut self,
      username: impl Into<String>,
    ) -> Result<Option<User>, UserRepositoryError> {
      Ok(self.users.get(&username.into()).cloned())
    }
  }

  #[tokio::test]
  async fn create_user_should_return_error_when_user_exists() {
    let mut user_repository = InMemoryUserRepository::default();

    let input = UserRegisterDto {
      username: String::from("username"),
      password: String::from("password"),
      email: String::from("user@gmail.com"),
      display_name: None,
    };

    user_repository
      .create(User {
        username: input.username.clone(),
        password: input.password.clone(),
        email: input.email.clone(),
        display_name: input.display_name.clone().unwrap_or_default(),
        ..Default::default()
      })
      .await
      .unwrap();

    let result = create_user(&mut user_repository, input).await;

    assert!(matches!(
      result,
      Err(CreateUserStoryError::UserAlreadyExists)
    ));
  }

  #[tokio::test]
  async fn create_user_should_return_user_when_user_is_created() {
    let mut user_repository = InMemoryUserRepository::default();

    let input = UserRegisterDto {
      username: String::from("username"),
      password: String::from("password"),
      email: String::from("user@gmail.com"),
      display_name: None,
    };

    let result = create_user(&mut user_repository, input.clone()).await;

    assert!(matches!(result, Ok(user) if user.username == input.username));
  }
}
