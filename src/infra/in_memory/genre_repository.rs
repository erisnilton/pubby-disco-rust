use std::collections::HashMap;

use crate::{
  domain::genre::{Genre, GenreRepository},
  AppState,
};

pub struct InMemoryGenreRepository {
  pub genres: HashMap<String, Genre>,
}

impl InMemoryGenreRepository {
  pub fn new(_: &AppState) -> Self {
    Self {
      genres: HashMap::new(),
    }
  }
}

impl GenreRepository for InMemoryGenreRepository {
  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<Genre>, crate::domain::genre::Error> {
    Ok(self.genres.get(&id.to_string()).cloned())
  }

  async fn create(
    &mut self,
    genre: Genre,
  ) -> Result<Genre, crate::domain::genre::Error> {
    self.genres.insert(genre.id.to_string(), genre.clone());
    Ok(genre)
  }

  async fn update(
    &mut self,
    genre: Genre,
  ) -> Result<Genre, crate::domain::genre::Error> {
    self.genres.insert(genre.id.to_string(), genre.clone());
    Ok(genre)
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::genre::Error> {
    self.genres.remove(&id.to_string());
    Ok(())
  }
}
