use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum UUIDError {
  InvalidUUID,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl From<UUID4> for uuid::Uuid {
  fn from(value: UUID4) -> Self {
    uuid::Uuid::parse_str(&value.0).unwrap()
  }
}

impl From<uuid::Uuid> for UUID4 {
  fn from(value: uuid::Uuid) -> Self {
    UUID4(value.to_string())
  }
}

impl Serialize for UUID4 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.0)
  }
}

impl<'de> Deserialize<'de> for UUID4 {
  fn deserialize<D>(deserializer: D) -> Result<UUID4, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    Ok(UUID4(s))
  }
}
