use crate::domain::{artist, genre};

#[derive(Debug, Clone)]
pub enum EntityUpdateError {
  Genre(genre::stories::apply_changes::Error),
  Artist(artist::stories::apply_changes::Error),
  Album(crate::domain::album::stories::apply_changes::Error),
}
