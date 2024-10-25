use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::ValidationError;

pub enum UUIDError {
  InvalidUUID,
}

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct UUID4(pub String);

impl UUID4 {
  pub fn new(value: impl Into<String>) -> Result<Self, UUIDError> {
    let value: String = value.into();
    if uuid::Uuid::parse_str(&value).is_err() {
      return Err(UUIDError::InvalidUUID);
    }

    Ok(Self(value))
  }

  pub fn generate() -> Self {
    Self(uuid::Uuid::new_v4().to_string())
  }
}

impl Display for UUID4 {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for UUID4 {
  fn default() -> Self {
    Self::generate()
  }
}
