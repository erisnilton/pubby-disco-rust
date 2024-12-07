pub trait PasswordHash {
  fn hash(&self, password: &str) -> String;
  fn verify(&self, password: &str, hash: &str) -> bool;
}
