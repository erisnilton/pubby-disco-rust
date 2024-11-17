use crate::{
  domain,
  shared::vo::{Slug, UUID4},
};

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateGenreDTO {
  #[validate(length(min = 1, max = 128))]
  name: String,

  slug: Option<Slug>,

  parent_id: Option<UUID4>,
}

impl From<CreateGenreDTO> for domain::genre::stories::contribute::CreateGenreInput {
  fn from(value: CreateGenreDTO) -> Self {
    domain::genre::stories::contribute::CreateGenreInput {
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

impl From<UpdateGenreDto> for domain::genre::contribution::changes::Changes {
  fn from(value: UpdateGenreDto) -> Self {
    Self {
      name: value.name,
      parent_id: value.parent_id,
      slug: value.slug,
    }
  }
}
