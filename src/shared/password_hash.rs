pub trait PasswordHash {
  fn hash_password(&self, password: &str) -> String;
  fn verify_password(&self, password: &str, hash: &str) -> bool;
}
