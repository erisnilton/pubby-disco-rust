use crate::{
  domain::{
    album::{dto::AlbumPresenter, AlbumEntity},
    artists::dto::CreateArtistDto,
    genre::dto::CreateGenreDto,
  },
  infra::actix::album::dto::CreateAlbumDTO,
  shared::vo::{CollaborativeEntityId, UpdateCollaborativeEntity},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CreateActivityEntityDto {
  Genre(CreateGenreDto),
  Artist(CreateArtistDto),
  Album(CreateAlbumDTO),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum CreateActivityDto {
  Create(CreateActivityEntityDto),
  Update {
    entity_id: CollaborativeEntityId,
    changes: UpdateCollaborativeEntity,
  },
}
