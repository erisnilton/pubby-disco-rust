use crate::app_state::AppState;

pub mod ports {
  use ports::password_hash::PasswordHash;

  pub fn password_hash() -> impl PasswordHash {
    infra::bcrypt::PasswordHash
  }
}

pub mod user {
  use super::*;

  pub fn user_repository(
    app_state: &AppState,
  ) -> impl application::user::repository::UserRepository {
    infra::sqlx::user::UserRepository::new(app_state.db.clone())
  }
}

pub mod genre {
  use super::*;

  pub fn genre_repository(
    app_state: &AppState,
  ) -> impl application::genre::repository::GenreRepository {
    infra::sqlx::genre::GenreRepository::new(app_state.db.clone())
  }
}

pub mod activity {
  use super::*;

  pub fn activity_repository(
    app_state: &AppState,
  ) -> impl application::activity::repository::ActivityRepository {
    infra::sqlx::activity::ActivityRepository::new(app_state.db.clone())
  }
}

pub mod album {
  use super::*;

  pub fn album_repository(
    app_state: &AppState,
  ) -> impl application::album::repository::AlbumRepository {
    infra::sqlx::album::AlbumRepository::new(app_state.db.clone())
  }
}

pub mod artist {
  use super::*;

  pub fn artist_repository(
    app_state: &AppState,
  ) -> impl application::artist::repository::ArtistRepository {
    infra::sqlx::artist::ArtistRepository::new(app_state.db.clone())
  }
}

pub mod media {
  use super::*;

  pub fn media_repository(
    app_state: &AppState,
  ) -> impl application::media::repository::MediaRepository {
    infra::sqlx::media::MediaRepository::new(app_state.db.clone())
  }
}

pub mod source {
  use super::*;

  pub fn source_repository(
    app_state: &AppState,
  ) -> impl application::source::repository::SourceRepository {
    infra::sqlx::source::SourceRepository::new(app_state.db.clone())
  }
}
