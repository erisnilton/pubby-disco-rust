use crate::{
  domain::{self, artists::dto::ArtistPresenter},
  infra,
};

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateArtistDTO {
  #[validate(length(min = 1, max = 128))]
  name: String,

  #[validate(length(min = 1, max = 128))]
  slug: String,

  #[validate(length(min = 1, max = 128))]
  country: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate, Default)]
pub struct UpdateArtistDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  #[validate(length(min = 1, max = 128))]
  pub slug: Option<String>,

  #[validate(length(min = 1, max = 128))]
  pub country: Option<String>,
}

impl From<CreateArtistDTO> for crate::domain::artists::dto::CreateArtistDto {
  fn from(value: infra::actix::artist::dto::CreateArtistDTO) -> Self {
    Self {
      name: value.name,
      slug: value.slug,
      country: value.country,
    }
  }
}

impl From<UpdateArtistDto> for crate::domain::artists::dto::UpdateArtistDto {
  fn from(value: UpdateArtistDto) -> Self {
    Self {
      name: value.name,
      slug: value.slug,
      country: value.country,
    }
  }
}

impl From<crate::domain::artists::dto::UpdateArtistDto> for UpdateArtistDto {
  fn from(value: crate::domain::artists::dto::UpdateArtistDto) -> Self {
    Self {
      name: value.name,
      slug: value.slug,
      country: value.country,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct GenrePresenter {
  id: String,
  name: String,
}

impl From<domain::artists::Artist> for ArtistPresenter {
  fn from(value: domain::artists::Artist) -> Self {
    Self {
      id: value.id.to_string(),
      name: value.name,
      slug: value.slug.to_string(),
      country: value.country,
      created_at: value.created_at,
      updated_at: value.updated_at,
    }
  }
}
