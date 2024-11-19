use crate::shared::password_hash::PasswordHash;

pub struct BcryptPasswordHash;

impl PasswordHash for BcryptPasswordHash {
  fn hash_password(&self, password: &str) -> String {
    bcrypt::hash(password, 10).unwrap_or_default()
  }

  fn verify_password(&self, password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
  }
}
