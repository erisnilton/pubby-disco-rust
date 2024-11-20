pub mod user {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxUserRepository as UserRepository;
  }
}

pub mod activity {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxActivityRepository as ActivityRepository;
  }
}

pub mod genre {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxGenreRepository as GenreRepository;
  }
}

pub mod artist {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxArtistRepository as ArtistRepository;
  }
}

pub mod album {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxAlbumRepository as AlbumRepository;
  }
}

pub mod media {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxMediaRepository as MediaRepository;
  }
}
