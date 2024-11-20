pub struct SqlxMediaRepository {
  db: sqlx::Pool<sqlx::Postgres>,
}

impl SqlxMediaRepository {
  pub fn new(app_state: &crate::AppState) -> Self {
    Self {
      db: app_state.db.clone(),
    }
  }
}

impl crate::domain::media::repository::MediaRepository for SqlxMediaRepository {
  async fn create(
    &mut self,
    _media: &crate::domain::media::Media,
  ) -> Result<(), crate::domain::media::repository::Error> {
    todo!()
  }

  async fn update(
    &mut self,
    _media: &crate::domain::media::Media,
  ) -> Result<(), crate::domain::media::repository::Error> {
    todo!()
  }

  async fn find_by_id(
    &mut self,
    _id: &crate::shared::vo::UUID4,
  ) -> Result<Option<crate::domain::media::Media>, crate::domain::media::repository::Error> {
    todo!()
  }

  async fn delete_by_id(
    &mut self,
    _id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::media::repository::Error> {
    todo!()
  }
}
