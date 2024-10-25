use validator::ValidationError;

pub fn uuid(uuid: &str) -> Result<(), ValidationError> {
  uuid::Uuid::parse_str(uuid)
    .map(|_| ())
    .map_err(|_| ValidationError::new("Invalid UUID"))
}
