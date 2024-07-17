use std::fmt::{self, Display, Formatter};

use serde::Serialize;

pub enum UUIDError {
    InvalidUUID,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct UUID4(pub String);

impl UUID4 {
    pub fn new(value: String) -> Result<Self, UUIDError> {
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