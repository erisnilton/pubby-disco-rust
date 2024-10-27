use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserRegisterDto {
  #[validate(length(min = 1, max = 80))]
  pub username: String,

  #[validate(length(min = 1, max = 128))]
  pub display_name: Option<String>,

  #[validate(length(min = 6, max = 255))]
  pub password: String,

  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserLoginDto {
  #[validate(length(min = 1))]
  pub username: String,

  #[validate(length(min = 1))]
  pub password: String,
}
