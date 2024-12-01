use std::collections::HashSet;

use crate::*;
use domain::{
  album::{album_aggregate::AlbumAggregate, Album},
  artist::Artist,
};
use shared::{
  paged::Paged,
  vo::{Slug, UUID4},
};
use sqlx::Postgres;

use super::many_to_many::ManyToManyBuilder;

pub struct SqlxAlbumRepository {
  db: sqlx::Pool<Postgres>,
}

impl SqlxAlbumRepository {
  pub fn new(app_state: &AppState) -> Self {
    Self {
      db: app_state.db.clone(),
    }
  }
}

impl domain::album::repository::AlbumRepository for SqlxAlbumRepository {
  async fn create(
    &mut self,
    album: &domain::album::Album,
  ) -> Result<domain::album::Album, domain::album::repository::Error> {
    let mut trx =
      self.db.begin().await.map_err(|error| {
        crate::domain::album::repository::Error::DatabaseError(error.to_string())
      })?;

    sqlx::query!(
      r#"
      INSERT INTO "album" ("id", "name", "album_type", "cover", "release_date", "parental_rating", "created_at", "updated_at")
      VALUES ($1, $2, $3::album_type, $4, $5, $6, $7, $8)
      "#,
      Into::<uuid::Uuid>::into(album.id().clone()),
      album.name(),
      album.album_type().to_string() as _,
      album.cover().clone(),
      album.release_date().clone(),
      (*album.parental_rating()) as i16,
      album.created_at(),
      album.updated_at()
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::repository::Error::DatabaseError(format!("Falha ao associar artistas: {}", err)))?;

    ManyToManyBuilder::new()
      .table("album_artist")
      .column("album_id")
      .related_column("artist_id")
      .build()
      .insert_many(&mut *trx, album.id(), album.artist_ids())
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    trx
      .commit()
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    Ok(album.clone())
  }

  async fn update(
    &mut self,
    album: &crate::domain::album::Album,
  ) -> Result<crate::domain::album::Album, crate::domain::album::repository::Error> {
    let mut trx = self
      .db
      .begin()
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    sqlx::query!(
      r#"
      UPDATE "album"
      SET "name" = $2, "cover" = $3, "release_date" = $4, "parental_rating" = $5, "updated_at" = $6
      WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(album.id().clone()),
      album.name(),
      album.cover().clone(),
      album.release_date().clone(),
      *album.parental_rating() as i16,
      album.updated_at()
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    ManyToManyBuilder::new()
      .table("album_artist")
      .column("album_id")
      .related_column("artist_id")
      .build()
      .update(&mut trx, album.id(), album.artist_ids())
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    trx
      .commit()
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    Ok(album.clone())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<crate::domain::album::Album>, crate::domain::album::repository::Error> {
    let result = sqlx::query!(
      r#"
      SELECT "id", "name", "album_type" as "album_type: String", "cover", "release_date", "parental_rating", "created_at", "updated_at"
      FROM "album"
      WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .fetch_optional(&self.db)
    .await
    .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    if result.is_none() {
      return Ok(None);
    }

    let result = result.unwrap();

    let artists_id = {
      let result = sqlx::query!(
        r#"
        SELECT "artist_id" FROM "album_artist" WHERE "album_id" = $1 
      "#,
        Into::<uuid::Uuid>::into(id.clone())
      )
      .fetch_all(&self.db)
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

      result
        .into_iter()
        .map(|value| shared::vo::UUID4::new(value.artist_id).unwrap())
        .collect::<HashSet<shared::vo::UUID4>>()
    };

    Ok(Some(
      domain::album::Album::builder()
        .id(shared::vo::UUID4::new(result.id).unwrap())
        .name(result.name)
        .album_type(result.album_type.parse().unwrap())
        .cover(result.cover)
        .parental_rating(result.parental_rating as u8)
        .release_date(result.release_date)
        .created_at(result.created_at)
        .updated_at(result.updated_at)
        .artist_ids(artists_id)
        .build(),
    ))
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::album::repository::Error> {
    let mut trx = self
      .db
      .begin()
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    sqlx::query!(
      r#"
      DELETE FROM album_artist WHERE album_id = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    sqlx::query!(
      r#"
      DELETE FROM album WHERE id = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&mut *trx)
    .await
    .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    trx
      .commit()
      .await
      .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    Ok(())
  }

  async fn find_by_slug(
    &mut self,
    slug: &shared::vo::Slug,
    artist_slug: &shared::vo::Slug,
  ) -> Result<
    Option<domain::album::album_aggregate::AlbumAggregate>,
    domain::album::repository::Error,
  > {
    let result = sqlx::query!(
      r#"
      select "a"."id", 
      "a"."name",
      "a"."album_type" as "album_type: String",
      "a"."cover",
      "a"."slug", 
      "a"."release_date", 
      "a"."parental_rating", 
      "a"."created_at", 
      "a"."updated_at",
      "a2"."id" as "artist_id: Option<uuid::Uuid>",
      "a2"."name" as "artist_name: Option<String>",
      "a2"."slug" as "artist_slug: Option<String>",
      "a2"."country" as "artist_country: Option<String>",
      "a2"."created_at" as "artist_created_at: Option<chrono::NaiveDateTime>",
      "a2"."updated_at" as "artist_update_at: Option<chrono::NaiveDateTime>"
      from "album" "a"
      left join "album_artist" "aa" on "a"."id" = "aa"."album_id" 
      left join "artist" "a2" on "a2"."id" = "aa"."artist_id"
      WHERE 
          "a"."id" IN (
          SELECT "album"."id"
          FROM "album"
          JOIN "album_artist" "aa" ON "album"."id" = "aa"."album_id"
          JOIN "artist" "a" ON "aa"."artist_id" = "a"."id"
          WHERE "album"."slug" = $1 AND "a"."slug" = $2
      )
    "#,
      slug.to_string(),
      artist_slug.to_string(),
    )
    .fetch_all(&self.db)
    .await
    .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))?;

    if result.is_empty() {
      return Ok(None);
    }

    let album = result.first().unwrap();

    let album_aggregate = AlbumAggregate::new(
      Album::builder()
        .id(UUID4::from(album.id))
        .name(album.name.clone())
        .cover(album.cover.clone())
        .slug(Slug::from(album.slug.clone()))
        .album_type(album.album_type.parse().unwrap())
        .created_at(album.created_at)
        .updated_at(album.updated_at)
        .build(),
      result
        .into_iter()
        .filter_map(|item| {
          item.artist_id.map(|id| {
            Artist::builder()
              .id(UUID4::from(id))
              .name(item.artist_name.unwrap_or_default())
              .slug(Slug::from(item.artist_slug.unwrap_or_default()))
              .country(item.artist_country.unwrap_or_default())
              .created_at(item.artist_created_at.unwrap_or_default())
              .updated_at(item.artist_update_at.unwrap_or_default())
              .build()
          })
        })
        .collect(),
    );

    Ok(Some(album_aggregate))
  }

  async fn find_by(
    &mut self,
    query: &domain::album::repository::FindAllQuery,
  ) -> Result<crate::shared::paged::Paged<AlbumAggregate>, domain::album::repository::Error> {
    let filter = create_filter!(domain::album::repository::FindAllQuery, qb => {
      search (value) => search_by!(qb, value.clone(), r#""a"."name""#, r#""a"."slug""#),
      artist_ids (value) => {
        qb.push(r#" "a"."id" in ( select "aa"."album_id" from album_artist aa where "aa"."artist_id" in ( "#);

        let mut separated = qb.separated(", ");
        for id in value {
          separated.push_bind(uuid::Uuid::from(id.clone()));
        }
        separated.push_unseparated(")");
      },
      name (value) => qb.push(r#" "a"."name" = "#).push_bind(value.to_string()),
      slug (value) => qb.push(r#" "a"."slug" = "#).push_bind(value.to_string()),
      album_type (value) => qb.push(r#" "a"."album_type" = "#).push_bind(value.to_string()),
      release_date (value) => qb.push(r#" "a"."release_date" = "#).push_bind(*value),
      min_release_date (value) => qb.push(r#" "a"."release_date" >= "#).push_bind(*value),
      max_release_date (value) => qb.push(r#" "a"."release_date" <= "#).push_bind(*value),
      parental_rating (value) => qb.push(r#" "a"."parental_rating" = "#).push_bind(*value as i16),
      min_parental_rating (value) => qb.push(r#" "a"."parental_rating" >= "#).push_bind(*value as i16),
      max_parental_rating (value) => qb.push(r#" "a"."parental_rating" <= "#).push_bind(*value as i16),
    });

    let (count, items) = tokio::join!(
      async {
        let mut query_builder = sqlx::QueryBuilder::new(r#"select count(id) from "album" "a" "#);
        filter(&mut query_builder, query);

        query_builder
          .build_query_scalar::<i64>()
          .fetch_one(&self.db)
          .await
          .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))
          .map(|count| count as usize)
      },
      async {
        #[derive(sqlx::FromRow)]
        struct Record {
          id: uuid::Uuid,
          name: String,
          album_type: String,
          cover: Option<String>,
          slug: String,
          release_date: Option<chrono::NaiveDate>,
          parental_rating: i16,
          created_at: chrono::NaiveDateTime,
          updated_at: chrono::NaiveDateTime,
          artist_id: Option<uuid::Uuid>,
          artist_name: String,
          artist_slug: String,
          artist_country: Option<String>,
          artist_created_at: chrono::NaiveDateTime,
          artist_updated_at: chrono::NaiveDateTime,
        }

        let mut query_builder = sqlx::QueryBuilder::new(
          r#"
          select 
            "a"."id",
            "a"."name",
            "a"."album_type"::text,
            "a"."cover",
            "a"."slug",
            "a"."release_date",
            "a"."parental_rating",
            "a"."created_at",
            "a"."updated_at",
            "ar"."id" as "artist_id",
            "ar"."name" as "artist_name",
            "ar"."slug" as "artist_slug",
            "ar"."country" as "artist_country",
            "ar"."created_at" as "artist_created_at",
            "ar"."updated_at" as "artist_updated_at"
            from "album" "a"
            left join "album_artist" "aa2" on "aa2"."album_id" = "a"."id"
            left join "artist" "ar" on "ar"."id" = "aa2"."artist_id" 
        "#,
        );

        filter(&mut query_builder, query);

        let page = crate::shared::paged::PageQueryParams::from(query.page.clone());

        query_builder
          .push(" LIMIT ")
          .push_bind(page.take as i16)
          .push(" OFFSET ")
          .push_bind(page.skip as i16);

        query_builder
          .build_query_as::<Record>()
          .fetch_all(&self.db)
          .await
          .map_err(|err| domain::album::repository::Error::DatabaseError(err.to_string()))
          .map(|result| {
            result
              .chunk_by(|a, b| a.id == b.id)
              .map(|result| {
                let album = result.first().unwrap();

                AlbumAggregate::new(
                  Album::builder()
                    .id(UUID4::from(album.id))
                    .album_type(album.album_type.parse().unwrap())
                    .name(album.name.clone())
                    .cover(album.cover.clone())
                    .slug(Slug::from(album.slug.clone()))
                    .parental_rating(album.parental_rating as u8)
                    .release_date(album.release_date)
                    .created_at(album.created_at)
                    .updated_at(album.updated_at)
                    .artist_ids(
                      result
                        .iter()
                        .filter_map(|row| row.artist_id.map(UUID4::from))
                        .collect(),
                    )
                    .build(),
                  result
                    .iter()
                    .filter_map(|row| {
                      row.artist_id.map(|id| {
                        Artist::builder()
                          .id(UUID4::from(id))
                          .name(row.artist_name.clone())
                          .slug(Slug::from(row.artist_slug.clone()))
                          .country(row.artist_country.clone())
                          .created_at(row.artist_created_at)
                          .updated_at(row.artist_updated_at)
                          .build()
                      })
                    })
                    .collect(),
                )
              })
              .collect()
          })
      }
    );

    Ok(Paged::from_tuple((count?, items?), query.page.clone()))
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use domain::{album::repository::AlbumRepository, artist::repository::ArtistRepository};
  use shared::vo::{Slug, UUID4};
  use sqlx::Postgres;

  use super::*;

  #[tokio::test]
  async fn test_create_album() {
    // Load .env file
    dotenvy::dotenv().ok();
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
        DELETE FROM "album_artist" WHERE album_id = '09c227b6-7498-4f63-b17c-11b7fe4e9c77'
      "#
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
        DELETE FROM "album" WHERE id = '09c227b6-7498-4f63-b17c-11b7fe4e9c77'
      "#,
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
        DELETE FROM "artist" WHERE id = '8115d6e2-e15f-42dc-8858-edd305805a7d' 
      "#,
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = infra::sqlx::SqlxArtistRepository::new(&app_state);
    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(UUID4::new("8115d6e2-e15f-42dc-8858-edd305805a7d").unwrap())
      .name(String::from("test_artist"))
      .slug(Slug::new("test-create-album-artist").unwrap())
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Erro ao criar artista");

    let album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new("09c227b6-7498-4f63-b17c-11b7fe4e9c77").unwrap())
      .name(String::from("test_album"))
      .artist_ids({
        let mut set = HashSet::new();
        set.insert(artist.id().clone());
        set
      })
      .build();

    let result = album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    let album_artist = sqlx::query!(r#"
        SELECT "album_id", "artist_id" FROM "album_artist" WHERE "album_id" = '09c227b6-7498-4f63-b17c-11b7fe4e9c77'
    "#).fetch_one(&app_state.db).await.expect("Falha ao consultar album_artist");

    cleanup(&app_state.db).await;

    assert_eq!(
      album_artist.artist_id,
      uuid::Uuid::from(artist.id().clone()),
      "Artista não foi relacionado ao album"
    );
    assert_eq!(
      result, album,
      "Album retornado não é o mesmo que foi criado"
    );
  }

  #[tokio::test]
  async fn update_album_info() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ALBUM_ID: &str = "26e948df-1351-461c-b45d-2eb183e6d6fc";

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
         DELETE FROM "album" WHERE "id" = $1
       "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name(String::from("test_album"))
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    let album = domain::album::AlbumBuilder::from(album)
      .name(String::from("test_album_updated"))
      .cover(Some(String::from("test_cover")))
      .parental_rating(16)
      .release_date(chrono::NaiveDate::from_ymd_opt(2021, 1, 1))
      .build();

    album_repository
      .update(&album)
      .await
      .expect("Erro ao atualizar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, album, "Album retornado não foi atualizado");
  }

  #[tokio::test]
  async fn update_album_set_artist() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ALBUM_ID: &str = "3fb46a9c-26b2-44f4-972a-16f32b0c8b5c";
    const ARTIST_ID: &str = "f6378582-8a71-4928-8edd-87af40b66446";

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = infra::sqlx::SqlxArtistRepository::new(&app_state);
    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let mut album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("test_artist"))
      .slug(shared::vo::Slug::new("test-update-album-set-artist").unwrap())
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Erro ao criar artista");

    album.artist_ids_mut().insert(artist.id().clone());

    album_repository
      .update(&album)
      .await
      .expect("Erro ao atualizar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, album, "Album retornado não foi atualizado");
  }

  #[tokio::test]
  async fn update_album_remove_artist() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ALBUM_ID: &str = "eb59a92e-5765-47b6-819d-68e1d30b996b";
    const ARTIST_ID: &str = "7d100457-6e92-498c-878c-bea6acb16f30";

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = infra::sqlx::SqlxArtistRepository::new(&app_state);
    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("test_artist"))
      .slug(shared::vo::Slug::new("test-update-album-remove-artist").unwrap())
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Erro ao criar artista");

    let mut album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .artist_ids({
        let mut set = HashSet::new();
        set.insert(artist.id().clone());
        set
      })
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    album.artist_ids_mut().clear();

    album_repository
      .update(&album)
      .await
      .expect("Erro ao atualizar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, album, "Album retornado não foi atualizado");
  }

  #[tokio::test]
  async fn update_album_replace_artist() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ALBUM_ID: &str = "a4dc9195-e3a1-493e-9482-7734454c4cbd";
    const ARTIST_ID: &str = "7681467f-b7ef-4035-a49d-a70d243799d3";
    const ARTIST_ID_2: &str = "7c99ddf2-6334-49ab-bfc6-ca14e7ea7a2e";

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id in ($1, $2)
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap(),
        uuid::Uuid::parse_str(ARTIST_ID_2).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = infra::sqlx::SqlxArtistRepository::new(&app_state);
    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("test_artist"))
      .slug(shared::vo::Slug::new("test-update-album-replace-artist").unwrap())
      .build();

    let artist_2 = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID_2).unwrap())
      .name(String::from("test_artist_2"))
      .slug(shared::vo::Slug::new("test-update-album-replace-artist_2").unwrap())
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Erro ao criar artista");

    artist_repository
      .create(&artist_2)
      .await
      .expect("Erro ao criar artista");

    let mut album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .artist_ids({
        let mut set = HashSet::new();
        set.insert(artist.id().clone());
        set
      })
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    let artists_ids = album.artist_ids_mut();

    artists_ids.clear();
    artists_ids.insert(artist_2.id().clone());

    album_repository
      .update(&album)
      .await
      .expect("Erro ao atualizar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, album, "Album retornado não foi atualizado");
  }

  #[tokio::test]
  async fn update_album_insert_artist() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ALBUM_ID: &str = "50bde96f-b94b-4c87-9d57-9fd305030634";
    const ARTIST_ID: &str = "d1fd1329-47c6-49f6-8756-527b6980270b";
    const ARTIST_ID_2: &str = "fce8bc1a-397a-4126-ab97-1f8a55875b07";

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id in ($1, $2)
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap(),
        uuid::Uuid::parse_str(ARTIST_ID_2).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = infra::sqlx::SqlxArtistRepository::new(&app_state);
    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("test_artist"))
      .slug(shared::vo::Slug::new("test-update-album-insert-artist").unwrap())
      .build();

    let artist_2 = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID_2).unwrap())
      .name(String::from("test_artist_2"))
      .slug(shared::vo::Slug::new("test-update-album-insert-artist_2").unwrap())
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Erro ao criar artista");

    artist_repository
      .create(&artist_2)
      .await
      .expect("Erro ao criar artista");

    let mut album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .artist_ids({
        let mut set = HashSet::new();
        set.insert(artist.id().clone());
        set
      })
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    album.artist_ids_mut().insert(artist_2.id().clone());

    album_repository
      .update(&album)
      .await
      .expect("Erro ao atualizar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, album);
  }

  #[tokio::test]
  async fn update_album_remove_a_artist() {
    // Load .env file
    dotenvy::dotenv().ok();

    const ALBUM_ID: &str = "123324fd-a42a-441b-b633-5ec7b1b715f7";
    const ARTIST_ID: &str = "46e7054e-dc41-4d72-a833-3524e748dd7e";
    const ARTIST_ID_2: &str = "6393a977-a557-4165-8d3d-abf34918bf98";

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
          DELETE FROM "artist" WHERE id in ($1, $2)
        "#,
        uuid::Uuid::parse_str(ARTIST_ID).unwrap(),
        uuid::Uuid::parse_str(ARTIST_ID_2).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");
    }
    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut artist_repository = infra::sqlx::SqlxArtistRepository::new(&app_state);
    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let artist = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID).unwrap())
      .name(String::from("test_artist"))
      .slug(shared::vo::Slug::new("test-update-album-remove-a-artist").unwrap())
      .build();

    let artist_2 = domain::artist::Artist::builder()
      .id(shared::vo::UUID4::new(ARTIST_ID_2).unwrap())
      .name(String::from("test_artist_2"))
      .slug(shared::vo::Slug::new("test-update-album-remove-a-artist_2").unwrap())
      .build();

    artist_repository
      .create(&artist)
      .await
      .expect("Erro ao criar artista");
    artist_repository
      .create(&artist_2)
      .await
      .expect("Erro ao criar artista");

    let mut album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .artist_ids({
        let mut set = HashSet::new();
        set.insert(artist.id().clone());
        set.insert(artist_2.id().clone());
        set
      })
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    album.artist_ids_mut().remove(artist_2.id());

    album_repository
      .update(&album)
      .await
      .expect("Erro ao atualizar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(result, album, "Album retornado não foi atualizado");
  }

  #[tokio::test]
  async fn delete_album() {
    const ALBUM_ID: &str = "a3f3dbb0-ae1f-40f3-a98d-7efc0c46bf05";

    // Load .env file
    dotenvy::dotenv().ok();

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    album_repository
      .delete_by_id(album.id())
      .await
      .expect("Erro ao excluir album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album");

    cleanup(&app_state.db).await;

    assert_eq!(result, None);
  }

  #[tokio::test]
  async fn find_album_by_id() {
    const ALBUM_ID: &str = "d91f0a0f-ecbd-4673-b18a-ad38076a7992";

    // Load .env file
    dotenvy::dotenv().ok();

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
          DELETE FROM "album_artist" WHERE album_id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album_artist");

      sqlx::query!(
        r#"
          DELETE FROM "album" WHERE id = $1
        "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");
    }

    let app_state = AppState::default().await;

    cleanup(&app_state.db).await;

    let mut album_repository = SqlxAlbumRepository::new(&app_state);

    let album = domain::album::Album::builder()
      .id(shared::vo::UUID4::new(ALBUM_ID).unwrap())
      .name("test_album".to_string())
      .build();

    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    let result = album_repository
      .find_by_id(album.id())
      .await
      .expect("Erro ao buscar album")
      .expect("Album não encontrado");

    cleanup(&app_state.db).await;

    assert_eq!(
      result, album,
      "Album retornado não é o mesmo que foi criado"
    );
  }
}
