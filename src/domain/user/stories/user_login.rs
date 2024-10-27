use crate::{
  domain::user::{User, UserRepository, UserRepositoryError},
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct Input {
  #[validate(length(min = 1))]
  pub username: String,

  #[validate(length(min = 1))]
  pub password: String,
}

pub async fn execute(
  user_repository: &mut impl UserRepository,
  password_hash: &impl PasswordHash,
  input: Input,
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
  use crate::domain::user::stories::user_login;
  use crate::infra::bcrypt::BcryptPasswordHash;

  use super::*;

  #[tokio::test]
  async fn test_should_login() {
    let mut user_repository = crate::infra::in_memory::InMemoryUserRepository::default();
    let password_hash = BcryptPasswordHash;
    let user = User {
      username: "test".to_string(),
      password: password_hash.hash_password("password"),
      email: "test@email.com".to_string(),
      display_name: String::new(),
      ..Default::default()
    };

    user_repository.create(user.clone()).await.unwrap();

    let result = user_login::execute(
      &mut user_repository,
      &password_hash,
      user_login::Input {
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
    let mut user_repository = crate::infra::in_memory::InMemoryUserRepository::default();
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
      let result = user_login::execute(
        &mut user_repository,
        &password_hash,
        Input {
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
