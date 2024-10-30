use crate::domain;

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateGenreDTO {
  #[validate(length(min = 1, max = 128))]
  name: String,

  #[validate(custom(function = "crate::shared::validator::uuid"))]
  parent_id: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, validator::Validate, Default)]
pub struct UpdateGenreDto {
  #[validate(length(min = 1, max = 128))]
  pub name: Option<String>,

  #[validate(length(min = 1, max = 128))]
  pub slug: Option<String>,

  #[validate(custom(function = "crate::shared::validator::uuid"))]
  pub parent_id: Option<String>,
}

impl From<CreateGenreDTO> for crate::domain::genre::dto::CreateGenreDto {
  fn from(value: CreateGenreDTO) -> Self {
    Self {
      name: value.name,
      parent_id: value.parent_id,
    }
  }
}

impl From<UpdateGenreDto> for crate::domain::genre::dto::UpdateGenreDto {
  fn from(value: UpdateGenreDto) -> Self {
    Self {
      name: value.name,
      slug: value.slug,
      parent_id: value.parent_id,
    }
  }
}

impl From<crate::domain::genre::dto::UpdateGenreDto> for UpdateGenreDto {
  fn from(value: crate::domain::genre::dto::UpdateGenreDto) -> Self {
    Self {
      name: value.name,
      slug: value.slug,
      parent_id: value.parent_id,
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct GenrePresenter {
  id: String,
  name: String,
}

impl From<domain::genre::Genre> for GenrePresenter {
  fn from(value: domain::genre::Genre) -> Self {
    Self {
      id: value.id.to_string(),
      name: value.name,
    }
  }
}
