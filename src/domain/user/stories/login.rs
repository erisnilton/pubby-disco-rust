use crate::{
  domain::user::{dto::UserLoginDto, User, UserRepository, UserRepositoryError},
  shared::password_hash::PasswordHash,
};

#[derive(Debug)]
pub enum LoginError {
  InvalidCredentials,
  RepositoryError(UserRepositoryError),
}

impl From<UserRepositoryError> for LoginError {
  fn from(value: UserRepositoryError) -> Self {
    LoginError::RepositoryError(value)
  }
}

pub async fn login(
  user_repository: &mut impl UserRepository,
  password_hash: &impl PasswordHash,
  input: UserLoginDto,
) -> Result<User, LoginError> {
  let user = user_repository.find_by_username(input.username).await?;

  if let Some(user) = user {
    if password_hash.verify_password(&input.password, &user.password) {
      return Ok(user);
    }
  }

  Err(LoginError::InvalidCredentials)
}

#[cfg(test)]
mod test {
  use crate::infra::bcrypt::BcryptPasswordHash;
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
  async fn test_should_login() {
    let mut user_repository = InMemoryUserRepository::default();
    let password_hash = BcryptPasswordHash;
    let user = User {
      username: "test".to_string(),
      password: password_hash.hash_password("password"),
      email: "test@email.com".to_string(),
      display_name: String::new(),
      ..Default::default()
    };

    user_repository.create(user.clone()).await.unwrap();

    let result = login(
      &mut user_repository,
      &password_hash,
      UserLoginDto {
        username: user.username.clone(),
        password: "password".to_string(),
      },
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, user.id);
  }

  #[tokio::test]
  async fn test_should_return_error_when_user_credentials_is_invalid() {
    let mut user_repository = InMemoryUserRepository::default();
    let password_hash = BcryptPasswordHash;

    let user = User {
      username: "test".to_string(),
      password: password_hash.hash_password("password"),
      email: "test@email.com".to_string(),
      display_name: String::new(),
      ..Default::default()
    };

    user_repository.create(user.clone()).await.unwrap();

    let credentials = vec![
      ("test_invalid", "password_invalid"),
      ("test", "password_invalid"),
      ("test_invalid", "password"),
    ];

    for (username, password) in credentials {
      let result = login(
        &mut user_repository,
        &password_hash,
        UserLoginDto {
          username: username.to_string(),
          password: password.to_string(),
        },
      )
      .await;

      assert!(
        matches!(result, Err(LoginError::InvalidCredentials)),
        "Expected InvalidCredentials error"
      );
    }
  }
}
