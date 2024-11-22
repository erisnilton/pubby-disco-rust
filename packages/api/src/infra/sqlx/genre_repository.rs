use crate::{
  domain::genre::{Error, Genre, GenreRepository},
  shared::vo::{Slug, UUID4},
  AppState,
};

pub struct SqlxGenreRepository {
  db: sqlx::Pool<sqlx::Postgres>,
}

impl SqlxGenreRepository {
  pub fn new(app_state: &AppState) -> Self {
    Self {
      db: app_state.db.clone(),
    }
  }
}

impl GenreRepository for SqlxGenreRepository {
  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<crate::domain::genre::Genre>, crate::domain::genre::Error> {
    let result = sqlx::query!(
      r#"
      SELECT "id", "name", "slug", "parent_id", "created_at", "updated_at"
      FROM "genre"
      WHERE "id" = $1
      LIMIT 1
    "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| Error::DatabaseError(err.to_string()))?;

    if let Some(row) = result {
      return Ok(Some(
        Genre::builder()
          .id(UUID4::new(row.id.to_string()).unwrap())
          .name(row.name)
          .slug(Slug::new(row.slug).unwrap())
          .parent_id(row.parent_id.map(|id| UUID4::new(id.to_string()).unwrap()))
          .created_at(row.created_at)
          .updated_at(row.updated_at)
          .build(),
      ));
    }

    Ok(None)
  }

  async fn create(&mut self, genre: &Genre) -> Result<(), Error> {
    let input = genre.clone();

    sqlx::query!(
      r#"
      INSERT INTO "genre" ("id", "name", "slug", "parent_id", "created_at", "updated_at")
      VALUES ($1, $2, $3, $4, $5, $6)
      "#,
      Into::<uuid::Uuid>::into(input.id().clone()),
      input.name(),
      input.slug().to_string(),
      input
        .parent_id()
        .clone()
        .map(|id| Into::<uuid::Uuid>::into(id)),
      input.created_at(),
      input.updated_at(),
    )
    .execute(&self.db)
    .await
    .map_err(|err| Error::DatabaseError(err.to_string()))?;
    Ok(())
  }

  async fn update(&mut self, genre: &Genre) -> Result<(), Error> {
    let input = genre.clone();

    sqlx::query!(
      r#"
      UPDATE "genre"
      SET "name" = $2, "slug" = $3, "parent_id" = $4, "updated_at" = $5
      WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(input.id().clone()),
      input.name(),
      input.slug().to_string(),
      input
        .parent_id()
        .clone()
        .map(|id| Into::<uuid::Uuid>::into(id)),
      input.updated_at(),
    )
    .execute(&self.db)
    .await
    .map_err(|err| Error::DatabaseError(err.to_string()))?;

    Ok(())
  }

  async fn delete_by_id(&mut self, id: &UUID4) -> Result<(), Error> {
    sqlx::query!(
      r#"DELETE FROM "genre" WHERE "id" = $1"#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&self.db)
    .await
    .map_err(|err| Error::DatabaseError(err.to_string()))?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{domain::genre::GenreBuilder, shared::vo::Slug, AppState};

  #[tokio::test]
  async fn test_create() {
    // Load .env file
    dotenvy::dotenv().ok();

    const GENRE_ID: &str = "698db80e-6d0e-4768-8a55-f9ff966f2523";

    async fn clean(db: &sqlx::pool::Pool<sqlx::Postgres>) {
      sqlx::query!(
        r#"DELETE FROM "genre" WHERE "id" = $1"#,
        uuid::Uuid::parse_str(GENRE_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Error cleaning database");
    }

    let app_state = AppState::default().await;

    let mut repository_genre = SqlxGenreRepository::new(&app_state);

    let genre = Genre::builder()
      .id(UUID4::new(GENRE_ID).unwrap())
      .name(String::from("Test"))
      .slug(Slug::new("test_create_genre").unwrap())
      .build();

    clean(&app_state.db).await;

    repository_genre
      .create(&genre)
      .await
      .expect("Falha ao criar Gênero");

    let result = repository_genre
      .find_by_id(genre.id())
      .await
      .expect("Falha ao buscar Gênero")
      .expect("Gênero não encontrado");

    clean(&app_state.db).await;

    assert_eq!(result, genre, "Gênero não é igual ao esperado");
  }

  #[tokio::test]
  async fn test_update() {
    // Load .env file
    dotenvy::dotenv().ok();

    const GENRE_ID: &str = "5c86933d-8e5c-4790-b692-f0562f788322";

    async fn clean(db: &sqlx::PgPool) {
      sqlx::query!(
        r#"DELETE FROM "genre" WHERE "id" = $1"#,
        uuid::Uuid::parse_str(GENRE_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao excluir dados de teste");
    }

    let app_state = AppState::default().await;

    let mut repository_genre = SqlxGenreRepository::new(&app_state);

    let genre = Genre::builder()
      .id(UUID4::new(GENRE_ID).unwrap())
      .name(String::from("Test"))
      .slug(Slug::new("test_update_genre").unwrap())
      .build();

    clean(&app_state.db).await;

    repository_genre.create(&genre).await.unwrap();

    let genre = GenreBuilder::from(genre)
      .name(String::from("Test 2"))
      .slug(Slug::new("test_update_genre2").unwrap())
      .build();

    repository_genre
      .update(&genre)
      .await
      .expect("Falha ao atualizar Gênero");

    let result = repository_genre
      .find_by_id(genre.id())
      .await
      .expect("Falha ao buscar Gênero")
      .expect("Gênero não encontrado");

    clean(&app_state.db).await;

    assert_eq!(result, genre, "Gênero não é igual ao esperado");
  }

  #[tokio::test]
  async fn test_delete() {
    // Load .env file
    dotenvy::dotenv().ok();

    const GENRE_ID: &str = "0dd24433-d16e-4b36-8f7e-27ce91c81f09";

    async fn clean(db: &sqlx::PgPool) {
      sqlx::query!(
        r#"DELETE FROM "genre" WHERE "id" = $1"#,
        uuid::Uuid::parse_str(GENRE_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao excluir dados de teste");
    }

    let app_state = AppState::default().await;

    let mut repository_genre = SqlxGenreRepository::new(&app_state);

    let genre = Genre::builder()
      .id(UUID4::new(GENRE_ID).unwrap())
      .name(String::from("Test"))
      .slug(Slug::new("test_delete_genre").unwrap())
      .build();

    clean(&app_state.db).await;

    repository_genre
      .create(&genre)
      .await
      .expect("Falha ao criar Gênero");

    repository_genre
      .delete_by_id(genre.id())
      .await
      .expect("Falha ao deletar Gênero");

    let result = repository_genre
      .find_by_id(genre.id())
      .await
      .expect("Falha ao buscar Gênero");

    assert_eq!(result, None, "Gênero não foi excluído");
  }
}
