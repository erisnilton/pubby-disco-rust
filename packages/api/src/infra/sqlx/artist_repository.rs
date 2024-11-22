use shared::{util::trim_datetime, vo::UUID4};
use sqlx::{query, Postgres};

use crate::*;

pub struct SqlxArtistRepository {
  db: sqlx::Pool<Postgres>,
}

impl SqlxArtistRepository {
  pub fn new(app_state: &AppState) -> Self {
    Self {
      db: app_state.db.clone(),
    }
  }
}

impl domain::artist::repository::ArtistRepository for SqlxArtistRepository {
  async fn create(
    &mut self,
    input: &crate::domain::artist::Artist,
  ) -> Result<crate::domain::artist::Artist, crate::domain::artist::repository::Error> {
    sqlx::query!(
      r#"
        INSERT INTO "artist" ("id", "name", "slug", "country", "created_at", "updated_at")
        VALUES ($1, $2, $3, $4, $5, $6)
      "#,
      Into::<uuid::Uuid>::into(input.id().clone()),
      input.name(),
      input.slug().to_string(),
      input.country().clone(),
      trim_datetime(*input.created_at()),
      trim_datetime(*input.updated_at()),
    )
    .execute(&self.db)
    .await
    .map_err(|err| domain::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(input.clone())
  }

  async fn update(
    &mut self,
    input: &crate::domain::artist::Artist,
  ) -> Result<crate::domain::artist::Artist, crate::domain::artist::repository::Error> {
    sqlx::query!(
      r#"
        UPDATE "artist"
        SET "name" = $2, "slug" = $3, "country" = $4, "updated_at" = $5
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(input.id().clone()),
      input.name(),
      input.slug().to_string(),
      input.country().clone(),
      trim_datetime(*input.updated_at()),
    )
    .execute(&self.db)
    .await
    .map_err(|err| domain::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<crate::domain::artist::Artist>, crate::domain::artist::repository::Error> {
    let artist = query!(
      r#"
        SELECT "id", "name", "slug", "country", "created_at", "updated_at"
        FROM "artist"
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone()),
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| domain::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(artist.map(|artist| {
      crate::domain::artist::Artist::builder()
        .id(shared::vo::UUID4::new(artist.id).unwrap_or_default())
        .name(artist.name)
        .slug(shared::vo::Slug::new(&artist.slug).unwrap_or_default())
        .country(artist.country)
        .created_at(trim_datetime(artist.created_at))
        .updated_at(trim_datetime(artist.updated_at))
        .build()
    }))
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::artist::repository::Error> {
    sqlx::query!(
      r#"
        DELETE FROM "artist"
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone()),
    )
    .execute(&self.db)
    .await
    .map_err(|err| domain::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(())
  }

  async fn find_by_slug(
    &mut self,
    slug: &shared::vo::Slug,
  ) -> Result<Option<crate::domain::artist::Artist>, crate::domain::artist::repository::Error> {
    let artist = query!(
      r#"
        SELECT "id", "name", "slug", "country", "created_at", "updated_at"
        FROM "artist"
        WHERE "slug" = $1
        LIMIT 1
      "#,
      slug.to_string(),
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| domain::artist::repository::Error::DatabaseError(err.to_string()))?;

    Ok(artist.map(|artist| {
      crate::domain::artist::Artist::builder()
        .id(UUID4::from(artist.id))
        .name(artist.name)
        .slug(shared::vo::Slug::new(&artist.slug).unwrap_or_default())
        .country(artist.country)
        .created_at(trim_datetime(artist.created_at))
        .updated_at(trim_datetime(artist.updated_at))
        .build()
    }))
  }
}

#[cfg(test)]
pub mod tests {
  use domain::artist::repository::ArtistRepository;

  use super::*;
  #[tokio::test]
  async fn test_create() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ARTIST_ID: &str = "7f2a8ffe-b276-4fb9-b119-fbd7aac8b4c8";

    async fn cleanup(db: &sqlx::PgPool) {
      sqlx::query!(
        "DELETE FROM artist WHERE id = $1",
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao limpar dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("Test"))
      .slug(shared::vo::Slug::new("test_create_artist").unwrap())
      .country(Some(String::from("BR")))
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Falha ao criar artista");

    let result = artist_repository
      .find_by_id(artist.id())
      .await
      .expect("Falha ao buscar artista")
      .expect("Artista não cadastrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, artist, "Os dados do artista não conferem");
  }

  #[tokio::test]

  async fn test_update() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ARTIST_ID: &str = "1001b01f-5a37-4220-9872-d53c63fc1cd3";

    async fn cleanup(db: &sqlx::PgPool) {
      sqlx::query!(
        "DELETE FROM artist WHERE id = $1",
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao limpar dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(&app_state);

    let mut artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("Test"))
      .slug(shared::vo::Slug::new("test_update_artist").unwrap())
      .country(Some(String::from("BR")))
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Falha ao criar artista");

    artist.set_name(String::from("Test Update"));

    artist_repository
      .update(&artist)
      .await
      .expect("Falha ao atualizar artista");

    let result = artist_repository
      .find_by_id(artist.id())
      .await
      .expect("Falha ao buscar artista")
      .expect("Artista não cadastrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, artist, "Os dados do artista não conferem");
  }

  #[tokio::test]
  async fn test_find_by_slug() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ARTIST_ID: &str = "128416be-f5f5-471d-8c2b-df1e353b5ced";

    async fn cleanup(db: &sqlx::PgPool) {
      sqlx::query!(
        "DELETE FROM artist WHERE id = $1",
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao limpar dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("Test"))
      .slug(shared::vo::Slug::new("test_find_by_slug").unwrap())
      .country(Some(String::from("BR")))
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Falha ao criar artista");

    let result = artist_repository
      .find_by_slug(artist.slug())
      .await
      .expect("Falha ao buscar artista")
      .expect("Artista não cadastrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, artist, "Os dados do artista não conferem");
  }

  #[tokio::test]
  async fn test_delete_by_id() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ARTIST_ID: &str = "f1485d4f-c2ca-4a25-bc7b-d5ffe487a15f";

    async fn cleanup(db: &sqlx::PgPool) {
      sqlx::query!(
        "DELETE FROM artist WHERE id = $1",
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao limpar dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("Test"))
      .slug(shared::vo::Slug::new("test_delete_by_id").unwrap())
      .country(Some(String::from("BR")))
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Falha ao criar artista");

    artist_repository
      .delete_by_id(artist.id())
      .await
      .expect("Falha ao deletar artista");

    let result = artist_repository
      .find_by_id(artist.id())
      .await
      .expect("Falha ao buscar artista");

    cleanup(&app_state.db).await;

    assert_eq!(result, None, "O artista não foi deletado");
  }

  #[tokio::test]
  async fn test_find_by_id() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ARTIST_ID: &str = "aab9b225-786b-46ee-b959-296b4ca3586b";

    async fn cleanup(db: &sqlx::PgPool) {
      sqlx::query!(
        "DELETE FROM artist WHERE id = $1",
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao limpar dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("Test"))
      .slug(shared::vo::Slug::new("test_find_by_id").unwrap())
      .country(Some(String::from("BR")))
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Falha ao criar artista");

    let result = artist_repository
      .find_by_id(artist.id())
      .await
      .expect("Falha ao buscar artista")
      .expect("Artista não cadastrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, artist, "Os dados do artista não conferem");
  }

  #[tokio::test]
  async fn test_find_by_id_not_found() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ARTIST_ID: &str = "78f71db5-89a7-4163-bf3a-6ef638107443";

    async fn cleanup(db: &sqlx::PgPool) {
      sqlx::query!(
        "DELETE FROM artist WHERE id = $1",
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Falha ao limpar dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = SqlxArtistRepository::new(&app_state);

    let result = artist_repository
      .find_by_id(&shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .await
      .expect("Falha ao buscar artista");

    cleanup(&app_state.db).await;

    assert_eq!(result, None, "O artista não foi encontrado");
  }
}
