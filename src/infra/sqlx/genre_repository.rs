use crate::{
  domain::genre::{Genre, GenreRepository, GenreRepositoryError},
  shared::vo::{Slug, UUID4},
};

pub struct SqlxGenreRepository {
  db: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Debug, sqlx::FromRow)]
struct GenreRecord {
  id: uuid::Uuid,
  name: String,
  slug: String,
  parent_id: Option<uuid::Uuid>,
  created_at: chrono::DateTime<chrono::Utc>,
  updated_at: chrono::DateTime<chrono::Utc>,
}

impl SqlxGenreRepository {
  pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
    Self { db }
  }
}

impl GenreRepository for SqlxGenreRepository {
  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<crate::domain::genre::Genre, crate::domain::genre::GenreRepositoryError> {
    let result: GenreRecord = sqlx::query_as(r#"SELECT * FROM "genre" WHERE "id" = $1 LIMIT 1"#)
      .bind(&id.0)
      .fetch_one(&self.db)
      .await
      .map_err(|err| GenreRepositoryError::InternalServerError(err.to_string()))?;

    Ok(Genre {
      id: UUID4::new(result.id.to_string()).unwrap_or_default(),
      name: result.name,
      parent_id: result
        .parent_id
        .map(|id| UUID4::new(id.to_string()).unwrap_or_default()),
      slug: Slug::new(&result.slug).unwrap_or_default(),
      created_at: result.created_at,
      updated_at: result.updated_at,
    })
  }
}
