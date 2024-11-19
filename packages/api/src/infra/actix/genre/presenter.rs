#[derive(Debug, serde::Serialize)]
pub struct GenrePresenter {
  id: String,
  name: String,
}

impl From<crate::domain::genre::Genre> for GenrePresenter {
  fn from(value: crate::domain::genre::Genre) -> Self {
    Self {
      id: value.id().to_string(),
      name: value.name().clone(),
    }
  }
}
