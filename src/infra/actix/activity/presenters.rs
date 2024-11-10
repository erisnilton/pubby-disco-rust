use chrono::Utc;

use crate::{
  domain,
  infra::actix::{
    collaborative_entity::dto::CollaborativeEntity, user::presenters::PublicUserPresenter,
  },
  shared,
};

#[derive(Debug, serde::Serialize)]
pub enum ActivityStatusDTO {
  Pending,
  Approved,
  Rejected(String),
  Draft,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum UpdateCollaborativeEntityDto {
  Genre(crate::infra::actix::genre::dto::UpdateGenreDto),
  Artist(crate::infra::actix::artist::dto::UpdateArtistDto),
  Album(crate::infra::actix::album::dto::UpdateAlbumDto),
}

impl From<shared::vo::UpdateCollaborativeEntity> for UpdateCollaborativeEntityDto {
  fn from(value: shared::vo::UpdateCollaborativeEntity) -> Self {
    match value {
      shared::vo::UpdateCollaborativeEntity::Genre(genre) => {
        UpdateCollaborativeEntityDto::Genre(genre.into())
      }
      shared::vo::UpdateCollaborativeEntity::Artist(artist) => {
        UpdateCollaborativeEntityDto::Artist(artist.into())
      }
      shared::vo::UpdateCollaborativeEntity::Album(album) => {
        UpdateCollaborativeEntityDto::Album(album.into())
      }
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub enum ActivityChange {
  Create(CollaborativeEntity),
  Update {
    entity: CollaborativeEntity,
    old_value: UpdateCollaborativeEntityDto,
    new_value: UpdateCollaborativeEntityDto,
  },
  Delete(CollaborativeEntity),
}

impl From<domain::activity::ActivityChange> for ActivityChange {
  fn from(value: domain::activity::ActivityChange) -> Self {
    match value {
      domain::activity::ActivityChange::Create(entity) => ActivityChange::Create(entity.into()),
      domain::activity::ActivityChange::Update {
        entity,
        old_value,
        new_value,
      } => ActivityChange::Update {
        entity: entity.into(),
        old_value: old_value.into(),
        new_value: new_value.into(),
      },
      domain::activity::ActivityChange::Delete(entity) => ActivityChange::Delete(entity.into()),
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct PublicActivityPresenter {
  pub id: String,
  pub status: ActivityStatusDTO,
  pub user: PublicUserPresenter,
  pub curator: Option<PublicUserPresenter>,
  pub revision_date: Option<chrono::DateTime<Utc>>,
  pub change: ActivityChange,
  pub created_at: chrono::DateTime<Utc>,
  pub updated_at: chrono::DateTime<Utc>,
}

impl From<crate::domain::activity::Activity> for PublicActivityPresenter {
  fn from(value: crate::domain::activity::Activity) -> Self {
    PublicActivityPresenter {
      id: value.id.to_string(),
      created_at: value.created_at,
      updated_at: value.updated_at,
      curator: value.curator.map(PublicUserPresenter::from),
      revision_date: value.revision_date,
      user: PublicUserPresenter::from(value.user),
      status: match value.status {
        crate::domain::activity::ActivityStatus::Pending => ActivityStatusDTO::Pending,
        crate::domain::activity::ActivityStatus::Approved => ActivityStatusDTO::Approved,
        crate::domain::activity::ActivityStatus::Rejected(reason) => {
          ActivityStatusDTO::Rejected(reason)
        }
        crate::domain::activity::ActivityStatus::Draft => ActivityStatusDTO::Draft,
      },
      change: value.change.into(),
    }
  }
}

impl From<crate::shared::paged::Paged<crate::domain::activity::Activity>>
  for crate::shared::paged::Paged<PublicActivityPresenter>
{
  fn from(value: crate::shared::paged::Paged<crate::domain::activity::Activity>) -> Self {
    Self {
      items: value.items.into_iter().map(|item| item.into()).collect(),
      page: value.page,
      total_items: value.total_items,
      total_pages: value.total_pages,
    }
  }
}
