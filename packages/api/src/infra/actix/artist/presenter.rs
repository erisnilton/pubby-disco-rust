use crate::{domain::artist::Artist, shared::vo::Slug};

#[derive(Debug, serde::Serialize)]
pub struct ArtistPresenter {
  id: String,
  name: String,
  slug: Slug,
  country: Option<String>,
}

impl From<crate::domain::artist::Artist> for ArtistPresenter {
  fn from(value: crate::domain::artist::Artist) -> Self {
    Self {
      id: value.id().to_string(),
      name: value.name().to_string(),
      slug: value.slug().clone(),
      country: value.country().clone(),
    }
  }
}

impl From<crate::shared::paged::Paged<Artist>> for crate::shared::paged::Paged<ArtistPresenter> {
  fn from(value: crate::shared::paged::Paged<Artist>) -> Self {
    Self {
      items: value.items.into_iter().map(ArtistPresenter::from).collect(),
      page: value.page,
      total_items: value.total_items,
      total_pages: value.total_pages,
    }
  }
}
