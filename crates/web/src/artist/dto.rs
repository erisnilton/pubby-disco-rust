use shared::vo::Slug;

#[derive(Debug, serde::Deserialize)]
pub struct FindAllQuery {
  pub search: Option<String>,
  pub country: Option<String>,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateArtistDTO {
  #[validate(length(min = 1, max = 128))]
  name: String,

  slug: Option<Slug>,

  #[validate(length(min = 1, max = 128))]
  country: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate, Default)]
pub struct UpdateArtistDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  pub slug: Option<Slug>,

  #[validate(length(min = 1, max = 128))]
  pub country: Option<String>,
}

impl From<CreateArtistDTO> for application::artist::stories::contribute::CreateArtistInput {
  fn from(value: CreateArtistDTO) -> Self {
    Self {
      name: value.name,
      country: value.country,
      slug: value.slug,
    }
  }
}

impl From<UpdateArtistDto> for domain::artist::vo::changes::Changes {
  fn from(value: UpdateArtistDto) -> Self {
    Self {
      name: value.name,
      country: value.country,
      slug: value.slug,
    }
  }
}
