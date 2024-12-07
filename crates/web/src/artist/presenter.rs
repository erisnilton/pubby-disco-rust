use domain::artist::entity::Artist;
use shared::vo::Slug;

#[derive(Debug, serde::Serialize)]
pub struct ArtistPresenter {
  id: String,
  name: String,
  slug: Slug,
  country: Option<String>,
}

impl From<Artist> for ArtistPresenter {
  fn from(value: Artist) -> Self {
    Self {
      id: value.id().to_string(),
      name: value.name().to_string(),
      slug: value.slug().clone(),
      country: value.country().clone(),
    }
  }
}
