use domain::artist::Artist;
use shared::vo::Slug;

use crate::*;

#[derive(Debug, serde::Deserialize)]
pub struct FindAllQuery {
  pub search: Option<String>,
  pub country: Option<String>,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateArtistDTO {
  #[validate(length(min = 1, max = 128))]
  name: String,

  slug: Option<crate::shared::vo::Slug>,

  #[validate(length(min = 1, max = 128))]
  country: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate, Default)]
pub struct UpdateArtistDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  pub slug: Option<crate::shared::vo::Slug>,

  #[validate(length(min = 1, max = 128))]
  pub country: Option<String>,
}

impl From<CreateArtistDTO> for crate::domain::artist::stories::contribute::CreateArtistInput {
  fn from(value: infra::actix::artist::dto::CreateArtistDTO) -> Self {
    Self {
      name: value.name,
      country: value.country,
      slug: value.slug,
    }
  }
}

impl From<UpdateArtistDto> for crate::domain::artist::contribution::changes::Changes {
  fn from(value: UpdateArtistDto) -> Self {
    Self {
      name: value.name,
      country: value.country,
      slug: value.slug,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct ArtistPresenter {
  id: String,
  name: String,
  slug: Slug,
  country: Option<String>,
}

impl From<domain::artist::Artist> for ArtistPresenter {
  fn from(value: domain::artist::Artist) -> Self {
    Self {
      id: value.id().to_string(),
      name: value.name().to_string(),
      slug: value.slug().clone(),
      country: value.country().clone(),
    }
  }
}

impl From<shared::paged::Paged<Artist>> for shared::paged::Paged<ArtistPresenter> {
  fn from(value: shared::paged::Paged<Artist>) -> Self {
    Self {
      items: value.items.into_iter().map(ArtistPresenter::from).collect(),
      page: value.page,
      total_items: value.total_items,
      total_pages: value.total_pages,
    }
  }
}
