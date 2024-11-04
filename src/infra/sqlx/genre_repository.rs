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
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
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
  ) -> Result<Option<crate::domain::genre::Genre>, crate::domain::genre::GenreRepositoryError> {
    let result: Result<GenreRecord, sqlx::Error> =
      sqlx::query_as(r#"SELECT * FROM "genre" WHERE "id" = $1 LIMIT 1"#)
        .bind(Into::<uuid::Uuid>::into(id.clone()))
        .fetch_one(&self.db)
        .await;

    match result {
      Ok(result) => Ok(Some(Genre {
        id: UUID4::new(result.id.to_string()).unwrap_or_default(),
        name: result.name,
        parent_id: result
          .parent_id
          .map(|id| UUID4::new(id.to_string()).unwrap_or_default()),
        slug: Slug::new(&result.slug).unwrap_or_default(),
        created_at: result.created_at.and_utc(),
        updated_at: result.updated_at.and_utc(),
      })),
      Err(sqlx::Error::RowNotFound) => Ok(None),
      Err(err) => Err(GenreRepositoryError::DatabaseError(err.to_string())),
    }
  }

  async fn create(&mut self, genre: Genre) -> Result<Genre, GenreRepositoryError> {
    let input = genre.clone();

    sqlx::query!(
      r#"
      INSERT INTO "genre" ("id", "name", "slug", "parent_id", "created_at", "updated_at")
      VALUES ($1, $2, $3, $4, $5, $6)
      "#,
      Into::<uuid::Uuid>::into(input.id.clone()),
      input.name,
      input.slug.to_string(),
      input.parent_id.map(|id| Into::<uuid::Uuid>::into(id)),
      input.created_at.naive_utc(),
      input.updated_at.naive_utc(),
    )
    .execute(&self.db)
    .await
    .map_err(|err| GenreRepositoryError::DatabaseError(err.to_string()))?;
    Ok(genre)
  }

  async fn update(&mut self, genre: Genre) -> Result<Genre, GenreRepositoryError> {
    let input = genre.clone();

    sqlx::query!(
      r#"
      UPDATE "genre"
      SET "name" = $2, "slug" = $3, "parent_id" = $4, "updated_at" = $5
      WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(input.id.clone()),
      input.name,
      input.slug.to_string(),
      input.parent_id.map(|id| Into::<uuid::Uuid>::into(id)),
      input.updated_at.naive_utc(),
    )
    .execute(&self.db)
    .await
    .map_err(|err| GenreRepositoryError::DatabaseError(err.to_string()))?;
    Ok(genre)
  }

  async fn delete_by_id(&mut self, id: &UUID4) -> Result<(), GenreRepositoryError> {
    sqlx::query!(
      r#"DELETE FROM "genre" WHERE "id" = $1"#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&self.db)
    .await
    .map_err(|err| GenreRepositoryError::DatabaseError(err.to_string()))?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{shared::vo::Slug, AppState};

  #[tokio::test]
  async fn test_create() {
    async fn clean(db: &sqlx::pool::Pool<sqlx::Postgres>) {
      sqlx::query!(r#"DELETE FROM "genre" WHERE "id" = '698db80e-6d0e-4768-8a55-f9ff966f2523'"#)
        .execute(db)
        .await
        .unwrap();
    }
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;

    let mut repository_genre = SqlxGenreRepository::new(app_state.db.clone());
    let genre = Genre {
      id: UUID4::new("698db80e-6d0e-4768-8a55-f9ff966f2523").unwrap(),
      name: "Test".to_string(),
      slug: Slug::new("test").unwrap(),
      ..Default::default()
    };
    clean(&app_state.db).await;
    let result = repository_genre.create(genre.clone()).await.unwrap();
    clean(&app_state.db).await;
    assert_eq!(result.id, genre.id);
    assert_eq!(result.name, genre.name);
    assert_eq!(result.slug, genre.slug);
  }

  #[tokio::test]
  async fn test_update() {
    async fn clean(db: &sqlx::pool::Pool<sqlx::Postgres>) {
      sqlx::query!(r#"DELETE FROM "genre" WHERE "id" = '5c86933d-8e5c-4790-b692-f0562f788322'"#)
        .execute(db)
        .await
        .unwrap();
    }
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;

    let mut repository_genre = SqlxGenreRepository::new(app_state.db.clone());
    let genre = Genre {
      id: UUID4::new("5c86933d-8e5c-4790-b692-f0562f788322").unwrap(),
      name: "Test".to_string(),
      slug: Slug::new("test_update").unwrap(),
      ..Default::default()
    };

    clean(&app_state.db).await;

    repository_genre.create(genre.clone()).await.unwrap();

    let genre = Genre {
      id: UUID4::new("5c86933d-8e5c-4790-b692-f0562f788322").unwrap(),
      name: "Test 2".to_string(),
      slug: Slug::new("test_update2").unwrap(),
      ..Default::default()
    };

    let result = repository_genre.update(genre.clone()).await.unwrap();

    clean(&app_state.db).await;

    assert_eq!(result.id, genre.id);
    assert_eq!(result.name, genre.name);
    assert_eq!(result.slug, genre.slug);
  }

  #[tokio::test]
  async fn test_delete() {
    async fn clean(db: &sqlx::pool::Pool<sqlx::Postgres>) {
      sqlx::query!(r#"DELETE FROM "genre" WHERE "id" = '0dd24433-d16e-4b36-8f7e-27ce91c81f09'"#)
        .execute(db)
        .await
        .unwrap();
    }
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;

    let mut repository_genre = SqlxGenreRepository::new(app_state.db.clone());
    let genre = Genre {
      id: UUID4::new("0dd24433-d16e-4b36-8f7e-27ce91c81f09").unwrap(),
      name: "Test".to_string(),
      slug: Slug::new("test_delete").unwrap(),
      ..Default::default()
    };

    clean(&app_state.db).await;

    repository_genre.create(genre.clone()).await.unwrap();

    repository_genre.delete_by_id(&genre.id).await.unwrap();

    let result = repository_genre.find_by_id(&genre.id).await.unwrap();

    assert_eq!(result, None);
  }
}
