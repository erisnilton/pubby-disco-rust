use domain::user::entity::User;
use ports::password_hash::PasswordHash;

use crate::user::repository::{Error, UserRepository};

#[derive(Debug)]
pub enum LoginError {
  InvalidCredentials,
  RepositoryError(Error),
}

impl From<Error> for LoginError {
  fn from(value: Error) -> Self {
    LoginError::RepositoryError(value)
  }
}

#[derive(Debug, Clone, validator::Validate)]
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
    if password_hash.verify(&input.password, user.password()) {
      return Ok(user);
    }
  }

  Err(LoginError::InvalidCredentials)
}

#[cfg(test)]
mod test {

  // #[tokio::test]
  // async fn test_should_login() {
  //   let mut user_repository = crate::infra::in_memory::InMemoryUserRepository::default();
  //   let password_hash = BcryptPasswordHash;
  //   let user = domain::user::User::builder()
  //     .username("test".to_string())
  //     .password(password_hash.hash_password("password"))
  //     .email(String::from("test@email.com"))
  //     .build();

  //   user_repository.create(&user).await.unwrap();

  //   let result = user_login::execute(
  //     &mut user_repository,
  //     &password_hash,
  //     user_login::Input {
  //       username: user.username().clone(),
  //       password: "password".to_string(),
  //     },
  //   )
  //   .await
  //   .expect("Falha ao logar");

  //   assert_eq!(
  //     result, user,
  //     "O usuário retornado não é o mesmo que foi criado"
  //   );
  // }

  // #[tokio::test]
  // async fn test_should_return_error_when_user_credentials_is_invalid() {
  //   let mut user_repository = crate::infra::in_memory::InMemoryUserRepository::default();
  //   let password_hash = BcryptPasswordHash;

  //   let user = domain::user::User::builder()
  //     .username("test".to_string())
  //     .password(password_hash.hash_password("password"))
  //     .email(String::from("test@email.com"))
  //     .build();

  //   user_repository.create(&user).await.unwrap();

  //   let credentials = vec![
  //     ("test_invalid", "password_invalid"),
  //     ("test", "password_invalid"),
  //     ("test_invalid", "password"),
  //   ];

  //   for (username, password) in credentials {
  //     let result = user_login::execute(
  //       &mut user_repository,
  //       &password_hash,
  //       Input {
  //         username: username.to_string(),
  //         password: password.to_string(),
  //       },
  //     )
  //     .await;

  //     assert!(
  //       matches!(result, Err(LoginError::InvalidCredentials)),
  //       "Expected InvalidCredentials error"
  //     );
  //   }
  // }
}
