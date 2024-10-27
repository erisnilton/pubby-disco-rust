pub mod user {
  pub mod repositories {
    pub use crate::infra::sqlx::SqlxUserRepository as UserRepository;
  }
}
