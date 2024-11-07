use crate::{
  domain::artists::dto::ArtistPresenter,
  infra::actix::genre::dto::GenrePresenter,
  shared::{self, vo::UUID4},
};

#[derive(Debug, serde::Deserialize)]
pub enum CreateActivityEntityDTO {
  Genre(crate::infra::actix::genre::dto::CreateGenreDTO),
  Artist(crate::infra::actix::artist::dto::CreateArtistDTO),
}

#[derive(Debug, serde::Deserialize)]
pub enum CollaborativeEntityId {
  Genre(String),
  Artist(String),
}

#[derive(Debug, serde::Serialize)]
pub enum CollaborativeEntity {
  Genre(GenrePresenter),
  Artist(ArtistPresenter),
}

impl From<shared::vo::CollaborativeEntity> for CollaborativeEntity {
  fn from(value: shared::vo::CollaborativeEntity) -> Self {
    match value {
      shared::vo::CollaborativeEntity::Default => panic!("Unexpected CollaborativeEntity::Default"),
      shared::vo::CollaborativeEntity::Genre(genre) => CollaborativeEntity::Genre(genre.into()),
      shared::vo::CollaborativeEntity::Artist(artist) => CollaborativeEntity::Artist(artist.into()),
    }
  }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum UpdateCollaborativeEntityDto {
  Genre(crate::infra::actix::genre::dto::UpdateGenreDto),
  Artist(crate::infra::actix::artist::dto::UpdateArtistDto),
}

impl From<CreateActivityEntityDTO> for crate::domain::activity::dto::CreateActivityEntityDto {
  fn from(value: CreateActivityEntityDTO) -> Self {
    match value {
      CreateActivityEntityDTO::Genre(dto) => {
        crate::domain::activity::dto::CreateActivityEntityDto::Genre(dto.into())
      }
      CreateActivityEntityDTO::Artist(dto) => {
        crate::domain::activity::dto::CreateActivityEntityDto::Artist(dto.into())
      }
    }
  }
}

impl From<CollaborativeEntityId> for crate::shared::vo::CollaborativeEntityId {
  fn from(value: CollaborativeEntityId) -> Self {
    match value {
      CollaborativeEntityId::Genre(id) => {
        crate::shared::vo::CollaborativeEntityId::Genre(UUID4::new(id).unwrap_or_default())
      }
      CollaborativeEntityId::Artist(id) => {
        crate::shared::vo::CollaborativeEntityId::Artist(UUID4::new(id).unwrap_or_default())
      }
    }
  }
}

impl From<UpdateCollaborativeEntityDto> for crate::shared::vo::UpdateCollaborativeEntityDto {
  fn from(value: UpdateCollaborativeEntityDto) -> Self {
    match value {
      UpdateCollaborativeEntityDto::Genre(dto) => {
        crate::shared::vo::UpdateCollaborativeEntityDto::Genre(dto.into())
      }
      UpdateCollaborativeEntityDto::Artist(dto) => {
        crate::shared::vo::UpdateCollaborativeEntityDto::Artist(dto.into())
      }
    }
  }
}
