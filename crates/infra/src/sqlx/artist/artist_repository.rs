use application::artist::repository::{ArtistRepository, FindAllQuery};
use domain::artist::entity::Artist;
use shared::{
  paged::{PageQueryParams, Paged},
  util::trim_datetime,
  vo::UUID4,
};

use ::sqlx;

use crate::*;

pub struct SqlxArtistRepository {
  db: sqlx::PgPool,
}

impl SqlxArtistRepository {
  pub fn new(db: sqlx::PgPool) -> Self {
    Self { db }
  }
}

impl ArtistRepository for SqlxArtistRepository {
  async fn create(
    &mut self,
    input: &Artist,
  ) -> Result<Artist, application::artist::repository::Error> {
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
    .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(input.clone())
  }

  async fn update(
    &mut self,
    input: &Artist,
  ) -> Result<Artist, application::artist::repository::Error> {
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
    .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(input.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &UUID4,
  ) -> Result<Option<Artist>, application::artist::repository::Error> {
    let artist = sqlx::query!(
      r#"
        SELECT "id", "name", "slug", "country", "created_at", "updated_at"
        FROM "artist"
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone()),
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(artist.map(|artist| {
      Artist::builder()
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
    id: &UUID4,
  ) -> Result<(), application::artist::repository::Error> {
    sqlx::query!(
      r#"
        DELETE FROM "artist"
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone()),
    )
    .execute(&self.db)
    .await
    .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))?;
    Ok(())
  }

  async fn find_by_slug(
    &mut self,
    slug: &shared::vo::Slug,
  ) -> Result<Option<Artist>, application::artist::repository::Error> {
    let artist = sqlx::query!(
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
    .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))?;

    Ok(artist.map(|artist| {
      Artist::builder()
        .id(UUID4::from(artist.id))
        .name(artist.name)
        .slug(shared::vo::Slug::new(&artist.slug).unwrap_or_default())
        .country(artist.country)
        .created_at(trim_datetime(artist.created_at))
        .updated_at(trim_datetime(artist.updated_at))
        .build()
    }))
  }

  async fn find_all(
    &mut self,
    query: &FindAllQuery,
  ) -> Result<Paged<Artist>, application::artist::repository::Error> {
    let artist_filter = create_filter!(FindAllQuery, builder => {
      country(value) => builder.push(r#""country" = "#).push_bind(value.clone()),
      search(value) => search_by!(builder, value.clone(), "name", "slug"),
    });

    let (count, items) = tokio::join!(
      async {
        let mut builder = sqlx::QueryBuilder::new(r#"SELECT COUNT("id") FROM "artist""#);

        artist_filter(&mut builder, query);

        builder
          .build_query_scalar::<Option<i64>>()
          .fetch_one(&self.db)
          .await
          .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))
          .map(|result| result.unwrap_or(0) as usize)
      },
      async {
        #[derive(sqlx::FromRow)]
        struct Record {
          id: uuid::Uuid,
          name: String,
          slug: String,
          country: Option<String>,
          created_at: chrono::NaiveDateTime,
          updated_at: chrono::NaiveDateTime,
        }
        let page_params = PageQueryParams::from(query.page.clone());

        let mut builder = sqlx::QueryBuilder::new(
          r#"SELECT "id", "name", "slug", "country", "created_at", "updated_at" FROM "artist""#,
        );

        artist_filter(&mut builder, query);

        builder
          .push(" LIMIT ")
          .push_bind(page_params.take as i32)
          .push(" OFFSET ")
          .push_bind(page_params.skip as i32);

        builder
          .build_query_as::<Record>()
          .fetch_all(&self.db)
          .await
          .map_err(|err| application::artist::repository::Error::DatabaseError(err.to_string()))
          .map(|result| {
            result
              .into_iter()
              .map(|row| {
                Artist::builder()
                  .id(UUID4::from(row.id))
                  .name(row.name)
                  .slug(shared::vo::Slug::from(row.slug))
                  .country(row.country)
                  .created_at(row.created_at)
                  .updated_at(row.updated_at)
                  .build()
              })
              .collect::<Vec<Artist>>()
          })
      }
    );
    Ok(Paged::from_tuple((count?, items?), query.page.clone()))
  }
}

#[cfg(test)]
pub mod tests {
  // use domain::artist::repository::ArtistRepository;

  // use super::*;
  // #[tokio::test]
  // async fn test_create() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const ARTIST_ID: &str = "7f2a8ffe-b276-4fb9-b119-fbd7aac8b4c8";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       "DELETE FROM artist WHERE id = $1",
  //       uuid::Uuid::parse_str(ARTIST_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao limpar dados de teste da tabela artist");
  //   }

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut artist_repository = SqlxArtistRepository::new(&app_state);

  //   let artist = Artist::builder()
  //     .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(shared::vo::Slug::new("test_create_artist").unwrap())
  //     .country(Some(String::from("BR")))
  //     .build();

  //   artist_repository
  //     .create(&artist)
  //     .await
  //     .expect("Falha ao criar artista");

  //   let result = artist_repository
  //     .find_by_id(artist.id())
  //     .await
  //     .expect("Falha ao buscar artista")
  //     .expect("Artista não cadastrado");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(result, artist, "Os dados do artista não conferem");
  // }

  // #[tokio::test]

  // async fn test_update() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const ARTIST_ID: &str = "1001b01f-5a37-4220-9872-d53c63fc1cd3";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       "DELETE FROM artist WHERE id = $1",
  //       uuid::Uuid::parse_str(ARTIST_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao limpar dados de teste da tabela artist");
  //   }

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut artist_repository = SqlxArtistRepository::new(&app_state);

  //   let mut artist = Artist::builder()
  //     .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(shared::vo::Slug::new("test_update_artist").unwrap())
  //     .country(Some(String::from("BR")))
  //     .build();

  //   artist_repository
  //     .create(&artist)
  //     .await
  //     .expect("Falha ao criar artista");

  //   artist.set_name(String::from("Test Update"));

  //   artist_repository
  //     .update(&artist)
  //     .await
  //     .expect("Falha ao atualizar artista");

  //   let result = artist_repository
  //     .find_by_id(artist.id())
  //     .await
  //     .expect("Falha ao buscar artista")
  //     .expect("Artista não cadastrado");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(result, artist, "Os dados do artista não conferem");
  // }

  // #[tokio::test]
  // async fn test_find_by_slug() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const ARTIST_ID: &str = "128416be-f5f5-471d-8c2b-df1e353b5ced";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       "DELETE FROM artist WHERE id = $1",
  //       uuid::Uuid::parse_str(ARTIST_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao limpar dados de teste da tabela artist");
  //   }

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut artist_repository = SqlxArtistRepository::new(&app_state);

  //   let artist = Artist::builder()
  //     .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(shared::vo::Slug::new("test_find_by_slug").unwrap())
  //     .country(Some(String::from("BR")))
  //     .build();

  //   artist_repository
  //     .create(&artist)
  //     .await
  //     .expect("Falha ao criar artista");

  //   let result = artist_repository
  //     .find_by_slug(artist.slug())
  //     .await
  //     .expect("Falha ao buscar artista")
  //     .expect("Artista não cadastrado");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(result, artist, "Os dados do artista não conferem");
  // }

  // #[tokio::test]
  // async fn test_delete_by_id() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const ARTIST_ID: &str = "f1485d4f-c2ca-4a25-bc7b-d5ffe487a15f";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       "DELETE FROM artist WHERE id = $1",
  //       uuid::Uuid::parse_str(ARTIST_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao limpar dados de teste da tabela artist");
  //   }

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut artist_repository = SqlxArtistRepository::new(&app_state);

  //   let artist = Artist::builder()
  //     .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(shared::vo::Slug::new("test_delete_by_id").unwrap())
  //     .country(Some(String::from("BR")))
  //     .build();

  //   artist_repository
  //     .create(&artist)
  //     .await
  //     .expect("Falha ao criar artista");

  //   artist_repository
  //     .delete_by_id(artist.id())
  //     .await
  //     .expect("Falha ao deletar artista");

  //   let result = artist_repository
  //     .find_by_id(artist.id())
  //     .await
  //     .expect("Falha ao buscar artista");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(result, None, "O artista não foi deletado");
  // }

  // #[tokio::test]
  // async fn test_find_by_id() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const ARTIST_ID: &str = "aab9b225-786b-46ee-b959-296b4ca3586b";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       "DELETE FROM artist WHERE id = $1",
  //       uuid::Uuid::parse_str(ARTIST_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao limpar dados de teste da tabela artist");
  //   }

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut artist_repository = SqlxArtistRepository::new(&app_state);

  //   let artist = Artist::builder()
  //     .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(shared::vo::Slug::new("test_find_by_id").unwrap())
  //     .country(Some(String::from("BR")))
  //     .build();

  //   artist_repository
  //     .create(&artist)
  //     .await
  //     .expect("Falha ao criar artista");

  //   let result = artist_repository
  //     .find_by_id(artist.id())
  //     .await
  //     .expect("Falha ao buscar artista")
  //     .expect("Artista não cadastrado");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(result, artist, "Os dados do artista não conferem");
  // }

  // #[tokio::test]
  // async fn test_find_by_id_not_found() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const ARTIST_ID: &str = "78f71db5-89a7-4163-bf3a-6ef638107443";

  //   async fn cleanup(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       "DELETE FROM artist WHERE id = $1",
  //       uuid::Uuid::parse_str(ARTIST_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao limpar dados de teste da tabela artist");
  //   }

  //   let app_state = AppState::default().await;

  //   cleanup(&app_state.db).await;

  //   let mut artist_repository = SqlxArtistRepository::new(&app_state);

  //   let result = artist_repository
  //     .find_by_id(&shared::vo::UUID4::new(ARTIST_ID).unwrap())
  //     .await
  //     .expect("Falha ao buscar artista");

  //   cleanup(&app_state.db).await;

  //   assert_eq!(result, None, "O artista não foi encontrado");
  // }
}
