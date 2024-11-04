use crate::domain::genre;

#[derive(Debug, Clone)]
pub enum EntityUpdateError {
  Genre(genre::stories::apply_changes::ApplyChangesError),
}
