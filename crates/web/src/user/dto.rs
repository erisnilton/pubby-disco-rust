#[derive(Debug, Clone, serde::Deserialize, validator::Validate)]
pub struct UserRegisterInputDTO {
  #[validate(length(min = 1, max = 80))]
  pub username: String,

  #[validate(length(min = 1, max = 128))]
  pub display_name: Option<String>,

  #[validate(length(min = 6, max = 255))]
  pub password: String,

  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Clone, serde::Deserialize, validator::Validate)]
pub struct UserLoginInputDTO {
  #[validate(length(min = 1))]
  pub username: String,

  #[validate(length(min = 1))]
  pub password: String,
}

impl From<UserRegisterInputDTO> for application::user::stories::user_register::Input {
  fn from(value: UserRegisterInputDTO) -> Self {
    Self {
      username: value.username,
      display_name: value.display_name,
      password: value.password,
      email: value.email,
    }
  }
}

impl From<UserLoginInputDTO> for application::user::stories::user_login::Input {
  fn from(value: UserLoginInputDTO) -> Self {
    Self {
      username: value.username,
      password: value.password,
    }
  }
}
