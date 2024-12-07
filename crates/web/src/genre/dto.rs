use shared::vo::{Slug, UUID4};

#[derive(Debug, serde::Deserialize)]
pub struct FindAllQuery {
  pub parent_id: Option<UUID4>,
  pub search: Option<String>,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateGenreDTO {
  #[validate(length(min = 1, max = 128))]
  name: String,

  slug: Option<Slug>,

  parent_id: Option<UUID4>,
}

impl From<CreateGenreDTO> for application::genre::stories::contribute::CreateGenreInput {
  fn from(value: CreateGenreDTO) -> Self {
    application::genre::stories::contribute::CreateGenreInput {
      name: value.name.clone(),
      parent_id: value.parent_id,
      slug: value.slug,
    }
  }
}

#[derive(Debug, Clone, serde::Deserialize, validator::Validate)]
pub struct UpdateGenreDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  pub parent_id: Option<Option<UUID4>>,

  pub slug: Option<Slug>,
}

impl From<UpdateGenreDto> for domain::genre::vo::Changes {
  fn from(value: UpdateGenreDto) -> Self {
    Self {
      name: value.name,
      parent_id: value.parent_id,
      slug: value.slug,
    }
  }
}
