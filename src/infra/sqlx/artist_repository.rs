use sqlx::{query, Postgres};

use crate::{
  domain::artists::repository::{ArtistRepository, ArtistRepositoryError},
  shared::vo::{Slug, UUID4},
};

pub struct SqlxArtistRepository {
  db: sqlx::Pool<Postgres>,
}

impl SqlxArtistRepository {
  pub fn new(db: sqlx::Pool<Postgres>) -> Self {
    Self { db }
  }
}

impl ArtistRepository for SqlxArtistRepository {
  async fn create(
    &mut self,
    input: &crate::domain::artists::Artist,
  ) -> Result<
    crate::domain::artists::Artist,
    crate::domain::artists::repository::ArtistRepositoryError,
  > {
    sqlx::query!(
      r#"
        INSERT INTO "artist" ("id", "name", "slug", "country", "created_at", "updated_at")
        VALUES ($1, $2, $3, $4, $5, $6)
      "#,
      Into::<uuid::Uuid>::into(input.id.clone()),
      input.name,
      input.slug.to_string(),
      input.country,
      input.created_at,
      input.updated_at,
    )
    .execute(&self.db)
    .await
    .map_err(|err| ArtistRepositoryError::DatabaseError(err.to_string()))?;
    Ok(input.clone())
  }

  async fn update(
    &mut self,
    input: &crate::domain::artists::Artist,
  ) -> Result<
    crate::domain::artists::Artist,
    crate::domain::artists::repository::ArtistRepositoryError,
  > {
    sqlx::query!(
      r#"
        UPDATE "artist"
        SET "name" = $2, "slug" = $3, "country" = $4, "updated_at" = $5
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(input.id.clone()),
      input.name,
      input.slug.to_string(),
      input.country,
      input.updated_at,
    )
    .execute(&self.db)
    .await
    .map_err(|err| ArtistRepositoryError::DatabaseError(err.to_string()))?;
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<
    Option<crate::domain::artists::Artist>,
    crate::domain::artists::repository::ArtistRepositoryError,
  > {
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
    .map_err(|err| ArtistRepositoryError::DatabaseError(err.to_string()))?;
    Ok(artist.map(|artist| crate::domain::artists::Artist {
      id: UUID4::new(artist.id).unwrap_or_default(),
      name: artist.name,
      slug: Slug::new(&artist.slug).unwrap_or_default(),
      country: artist.country.unwrap_or_default(),
      created_at: artist.created_at,
      updated_at: artist.updated_at,
    }))
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::artists::repository::ArtistRepositoryError> {
    sqlx::query!(
      r#"
        DELETE FROM "artist"
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone()),
    )
    .execute(&self.db)
    .await
    .map_err(|err| ArtistRepositoryError::DatabaseError(err.to_string()))?;
    Ok(())
  }

  async fn find_by_slug(
    &mut self,
    slug: &Slug,
  ) -> Result<
    crate::domain::artists::Artist,
    crate::domain::artists::repository::ArtistRepositoryError,
  > {
    let artist = query!(
      r#"
        SELECT "id", "name", "slug", "country", "created_at", "updated_at"
        FROM "artist"
        WHERE "slug" = $1
      "#,
      slug.to_string(),
    )
    .fetch_one(&self.db)
    .await
    .map_err(|err| ArtistRepositoryError::DatabaseError(err.to_string()))?;
    Ok(crate::domain::artists::Artist {
      id: UUID4::new(artist.id).unwrap_or_default(),
      name: artist.name,
      slug: Slug::new(&artist.slug).unwrap_or_default(),
      country: artist.country.unwrap_or_default(),
      created_at: artist.created_at,
      updated_at: artist.updated_at,
    })
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::{
    domain::{artists::repository::ArtistRepository, artists::Artist},
    AppState,
  };

  #[tokio::test]
  async fn test_create() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());

    let artist = Artist::new(
      "Test".to_string(),
      Slug::new("test").unwrap(),
      "BR".to_string(),
    );

    let artist = artist_repository.create(&artist).await.unwrap();

    assert_eq!(artist.name, "Test");
    assert_eq!(artist.slug, Slug::new("test").unwrap());
    assert_eq!(artist.country, "BR");
  }

  #[tokio::test]

  async fn test_update() {
    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;

    let mut artist_repository = SqlxArtistRepository::new(app_state.db.clone());

    let artist = Artist::new(
      "Test".to_string(),
      Slug::new("test").unwrap(),
      "BR".to_string(),
    );

    let artist = artist_repository.create(&artist).await.unwrap();

    let artist = artist_repository
      .find_by_id(&artist.id)
      .await
      .unwrap()
      .unwrap();

    let artist = artist_repository.update(&artist).await.unwrap();

    assert_eq!(artist.name, "Test");
    assert_eq!(artist.slug, Slug::new("test").unwrap());
    assert_eq!(artist.country, "BR");
  }
}
