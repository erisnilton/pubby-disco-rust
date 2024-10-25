use chrono::Utc;
use serde::Serialize;

use super::Artist;

#[derive(Debug, Serialize)]
pub struct ArtistPresenterDTO {
  pub id: String,
  pub slug: String,
  pub name: String,
  pub country: String,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl From<Artist> for ArtistPresenterDTO {
  fn from(artist: Artist) -> Self {
    Self {
      id: format!("{}", artist.id),
      slug: artist.slug,
      name: artist.name,
      country: artist.country,
      created_at: artist.created_at,
      updated_at: artist.updated_at,
    }
  }
}
