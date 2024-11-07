use crate::domain::{artists, genre};

#[derive(Debug, Clone)]
pub enum EntityUpdateError {
  Genre(genre::stories::apply_changes::ApplyChangesError),
  Artist(artists::stories::apply_changes::ApplyChangesError),
}
