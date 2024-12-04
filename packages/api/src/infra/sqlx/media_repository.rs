use crate::*;

use domain::{
  artist::Artist,
  media::{media_aggregate::MediaAggregate, Media},
  source::source_entity::Source,
};
use shared::{
  paged::{PageQueryParams, Paged},
  vo::{Slug, UUID4},
};

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

fn to_database_error(error: sqlx::error::Error) -> crate::domain::media::repository::Error {
  crate::domain::media::repository::Error::DatabaseError(error.to_string())
}

impl crate::domain::media::repository::MediaRepository for SqlxMediaRepository {
  async fn create(
    &mut self,
    media: &crate::domain::media::Media,
  ) -> Result<(), crate::domain::media::repository::Error> {
    let mut trx = self.db.begin().await.map_err(to_database_error)?;

    sqlx::query!(
      r#"
      INSERT INTO "media" ("id", "name", "type", "slug", "cover", "release_date", "is_single", "parental_rating", "created_at", "updated_at")
      VALUES($1,$2, $3::"media_type", $4, $5, $6, $7, $8, $9, $10)
    "#,
    Into::<uuid::Uuid>::into(media.id().clone()),
    media.name().clone(),
    media.media_type().to_string() as _,
    media.slug().to_string(),
    media.cover().clone(),
    media.release_date().clone(),
    *media.is_single(),
    *media.parental_rating() as i16,
    media.created_at().clone(),
    media.updated_at().clone()

    )
    .execute(&mut *trx)
    .await.map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_album")
      .column("media_id")
      .related_column("album_id")
      .build()
      .insert_many(&mut *trx, media.id(), media.album_ids())
      .await
      .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_genre")
      .column("media_id")
      .related_column("genre_id")
      .build()
      .insert_many(&mut *trx, media.id(), media.genre_ids())
      .await
      .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_composer")
      .column("media_id")
      .related_column("composer_id")
      .build()
      .insert_many(&mut *trx, media.id(), media.composer_ids())
      .await
      .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_interpreter")
      .column("media_id")
      .related_column("interpreter_id")
      .build()
      .insert_many(&mut *trx, media.id(), media.interpreter_ids())
      .await
      .map_err(to_database_error)?;

    trx.commit().await.map_err(to_database_error)?;

    Ok(())
  }

  async fn update(
    &mut self,
    media: &crate::domain::media::Media,
  ) -> Result<(), crate::domain::media::repository::Error> {
    let mut trx = self.db.begin().await.map_err(to_database_error)?;

    sqlx::query!(
      r#"
        UPDATE "media"
        SET "name" = $2, "type" = $3::"media_type", "slug" = $4, "cover" = $5, "release_date" = $6, "is_single" = $7, "parental_rating" = $8, "updated_at" = $9
        WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(media.id().clone()),
      media.name().clone(),
      media.media_type().to_string() as _,
      media.slug().to_string(),
      media.cover().clone(),
      media.release_date().clone(),
      *media.is_single(),
      *media.parental_rating() as i16,
      media.updated_at().clone()
    )
    .execute(&mut *trx)
    .await
    .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_album")
      .column("media_id")
      .related_column("album_id")
      .build()
      .update(&mut trx, media.id(), media.album_ids())
      .await
      .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_genre")
      .column("media_id")
      .related_column("genre_id")
      .build()
      .update(&mut trx, media.id(), media.genre_ids())
      .await
      .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_interpreter")
      .column("media_id")
      .related_column("interpreter_id")
      .build()
      .update(&mut trx, media.id(), media.interpreter_ids())
      .await
      .map_err(to_database_error)?;

    super::many_to_many::ManyToManyBuilder::new()
      .table("media_composer")
      .column("media_id")
      .related_column("composer_id")
      .build()
      .update(&mut trx, media.id(), media.composer_ids())
      .await
      .map_err(to_database_error)?;

    trx.commit().await.map_err(to_database_error)?;

    Ok(())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<Option<crate::domain::media::Media>, crate::domain::media::repository::Error> {
    let media = sqlx::query!(
      r#"
        SELECT "id", "name", "type" as "media_type: String","slug", "cover", "release_date", "is_single", "parental_rating", "created_at", "updated_at" FROM "media" WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .fetch_optional(&self.db)
    .await
    .map_err(to_database_error)?
    .map(|value| crate::domain::media::Media::builder()
    .id(crate::shared::vo::UUID4::from(value.id))
    .name(value.name)
    .media_type(value.media_type.parse().unwrap())
    .slug(crate::shared::vo::Slug::new(value.slug).unwrap())
    .cover(value.cover)
    .is_single(value.is_single)
    .release_date(value.release_date)
    .parental_rating(value.parental_rating as u8)
    .created_at(value.created_at)
    .updated_at(value.updated_at)
    .build());

    if media.is_none() {
      return Ok(None);
    }

    let mut media = media.unwrap();

    let (album_ids, genre_ids, composer_ids, interpreter_ids) = tokio::join!(
      async {
        sqlx::query_scalar!(
          r#" SELECT "album_id" FROM "media_album" WHERE "media_id" = $1 "#,
          uuid::Uuid::from(id.clone())
        )
        .fetch_all(&self.db)
        .await
        .map_err(to_database_error)
        .map(|result| {
          result
            .into_iter()
            .map(crate::shared::vo::UUID4::from)
            .collect::<std::collections::HashSet<crate::shared::vo::UUID4>>()
        })
      },
      async {
        sqlx::query_scalar!(
          r#" SELECT "genre_id" FROM "media_genre" WHERE "media_id" = $1 "#,
          uuid::Uuid::from(id.clone())
        )
        .fetch_all(&self.db)
        .await
        .map_err(to_database_error)
        .map(|result| {
          result
            .into_iter()
            .map(crate::shared::vo::UUID4::from)
            .collect::<std::collections::HashSet<crate::shared::vo::UUID4>>()
        })
      },
      async {
        sqlx::query_scalar!(
          r#" SELECT "composer_id" FROM "media_composer" WHERE "media_id" = $1 "#,
          uuid::Uuid::from(id.clone())
        )
        .fetch_all(&self.db)
        .await
        .map_err(to_database_error)
        .map(|result| {
          result
            .into_iter()
            .map(crate::shared::vo::UUID4::from)
            .collect::<std::collections::HashSet<crate::shared::vo::UUID4>>()
        })
      },
      async {
        sqlx::query_scalar!(
          r#" SELECT "interpreter_id" FROM "media_interpreter" WHERE "media_id" = $1 "#,
          uuid::Uuid::from(id.clone())
        )
        .fetch_all(&self.db)
        .await
        .map_err(to_database_error)
        .map(|result| {
          result
            .into_iter()
            .map(crate::shared::vo::UUID4::from)
            .collect::<std::collections::HashSet<crate::shared::vo::UUID4>>()
        })
      }
    );

    media = crate::domain::media::MediaBuilder::from(media)
      .album_ids(album_ids?)
      .composer_ids(composer_ids?)
      .genre_ids(genre_ids?)
      .interpreter_ids(interpreter_ids?)
      .build();

    Ok(Some(media))
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::media::repository::Error> {
    sqlx::query!(
      r#"DELETE FROM "media" WHERE "id" = $1"#,
      uuid::Uuid::from(id.clone())
    )
    .execute(&self.db)
    .await
    .map_err(to_database_error)?;
    Ok(())
  }
  async fn find_by(
    &mut self,
    query: &crate::domain::media::repository::FindByQuery,
  ) -> Result<
    crate::shared::paged::Paged<crate::domain::media::media_aggregate::MediaAggregate>,
    crate::domain::media::repository::Error,
  > {
    let filter = create_filter!(crate::domain::media::repository::FindByQuery, qb => {
      search(value) => search_by!(qb, value.clone(), r#""m"."name""#, r#""m"."slug""#),
      release_date(value) => qb.push(r#""m"."release_date" = "#).push_bind(*value),
      min_release_date(value) => qb.push(r#""m"."release_date" >= "#).push_bind(*value),
      max_release_date(value) => qb.push(r#""m"."release_date" <= "#).push_bind(*value),
      parental_rating(value) => qb.push(r#""m"."parental_rating" = "#).push_bind(*value as i16),
      min_parental_rating(value) => qb.push(r#""m"."parental_rating" >= "#).push_bind(*value as i16),
      max_parental_rating(value) => qb.push(r#""m"."parental_rating" <= "#).push_bind(*value as i16),
      is_single(value) => qb.push(r#""m"."is_single" = "#).push_bind(*value),
      media_type(value) => qb.push(r#""m"."type" = "#).push_bind(value.to_string()),
      slug(value) => qb.push(r#""m"."slug" = "#).push_bind(value.to_string()),
      artist_ids(value) => {
        qb.push(r#""m"."id" IN (SELECT "mi"."media_id" FROM "media_interpreter" "mi" WHERE "mi"."interpreter_id" IN ("#);

        let mut separated = qb.separated(", ");

        for id in value {
          separated.push_bind(uuid::Uuid::from(id.clone()));
        }

        separated.push_unseparated("))");
      },
      composer_ids(value) => {
        qb.push(r#""m"."id" IN (SELECT "mc"."media_id" FROM "media_composer" "mc" WHERE "mc"."composer_id" IN ("#);

        let mut separated = qb.separated(", ");

        for id in value {
          separated.push_bind(uuid::Uuid::from(id.clone()));
        }

        separated.push_unseparated("))");
      },
      genre_ids(value) => {
        qb.push(r#""m"."id" IN (SELECT "mg"."media_id" FROM "media_genre" "mg" WHERE "mg"."genre_id" IN ("#);

        let mut separated = qb.separated(", ");

        for id in value {
          separated.push_bind(uuid::Uuid::from(id.clone()));
        }

        separated.push_unseparated("))");
      },
      album_ids(value) => {
        qb.push(r#""m"."id" IN (SELECT "ma"."media_id" FROM "media_album" "ma" WHERE "ma"."album_id" IN ("#);

        let mut separated = qb.separated(", ");

        for id in value {
          separated.push_bind(uuid::Uuid::from(id.clone()));
        }

        separated.push_unseparated("))");
      },
    });

    let (count, items) = tokio::join!(
      async {
        let mut query_builder = sqlx::QueryBuilder::new(r#"SELECT COUNT("id") FROM "media""#);
        filter(&mut query_builder, query);

        query_builder
          .build_query_scalar::<i64>()
          .fetch_one(&self.db)
          .await
          .map_err(to_database_error)
          .map(|count| count as usize)
      },
      async {
        #[derive(Debug, Clone, sqlx::FromRow)]
        struct Record {
          id: uuid::Uuid,
          name: String,
          media_type: String,
          slug: String,
          release_date: Option<chrono::NaiveDate>,
          cover: Option<String>,
          parental_rating: i16,
          is_single: bool,
          created_at: chrono::NaiveDateTime,
          updated_at: chrono::NaiveDateTime,
        }
        let mut query_builder = sqlx::QueryBuilder::new(
          r#"
            select 
            "m"."id",
            "m"."name", 
            "m"."type"::text as "media_type", 
            "m"."slug", 
            "m"."release_date",
            "m"."cover",
            "m"."parental_rating",
            "m"."is_single",
            "m"."created_at",
            "m"."updated_at"
            from "media" "m" 
          "#,
        );

        filter(&mut query_builder, query);

        let page = PageQueryParams::from(query.page.clone());

        query_builder
          .push(" LIMIT ")
          .push_bind(page.take as i64)
          .push(" OFFSET ")
          .push_bind(page.skip as i64)
          .build_query_as::<Record>()
          .fetch_all(&self.db)
          .await
          .map_err(to_database_error)
          .map(|result| {
            result
              .into_iter()
              .map(|media| {
                let db = &self.db;

                async move {
                  let (composers, interpreters, sources) = tokio::join!(
                    async {
                      sqlx::query!(
                        r#"
                        SELECT
                        "a"."id",
                        "a"."name",
                        "a"."slug",
                        "a"."country",
                        "a"."created_at",
                        "a"."updated_at"
                        FROM "media_composer" "mc"
                        LEFT JOIN "artist" "a" ON "a"."id" = "mc"."composer_id"
                        WHERE mc."media_id" = $1
                        "#,
                        media.id.clone()
                      )
                      .fetch_all(db)
                      .await
                      .map_err(to_database_error)
                      .map(|result| {
                        result
                          .into_iter()
                          .map(|result| {
                            Artist::builder()
                              .id(UUID4::from(result.id))
                              .name(result.name)
                              .slug(Slug::from(result.slug))
                              .country(result.country)
                              .created_at(result.created_at)
                              .updated_at(result.updated_at)
                              .build()
                          })
                          .collect::<Vec<_>>()
                      })
                    },
                    async {
                      sqlx::query!(
                        r#"
                        SELECT
                        "a"."id",
                        "a"."name",
                        "a"."slug",
                        "a"."country",
                        "a"."created_at",
                        "a"."updated_at"
                        FROM "media_interpreter" "mi"
                        LEFT JOIN "artist" "a" ON "a"."id" = "mi"."interpreter_id"
                        WHERE mi."media_id" = $1
                        "#,
                        media.id.clone()
                      )
                      .fetch_all(db)
                      .await
                      .map_err(to_database_error)
                      .map(|result| {
                        result
                          .into_iter()
                          .map(|result| {
                            Artist::builder()
                              .id(UUID4::from(result.id))
                              .name(result.name)
                              .slug(Slug::from(result.slug))
                              .country(result.country)
                              .created_at(result.created_at)
                              .updated_at(result.updated_at)
                              .build()
                          })
                          .collect::<Vec<_>>()
                      })
                    },
                    async {
                      sqlx::query!(
                        r#"
                          SELECT
                          "s"."id",
                          "s"."source_type"::TEXT,
                          "s"."src",
                          "s"."media_id",
                          "s"."created_at",
                          "s"."updated_at"
                          FROM "source" "s"
                          WHERE "s"."media_id" = $1
                          "#,
                        media.id.clone()
                      )
                      .fetch_all(db)
                      .await
                      .map_err(to_database_error)
                      .map(|result| {
                        result
                          .into_iter()
                          .map(|result| {
                            Source::builder()
                              .id(UUID4::from(result.id))
                              .src(result.src)
                              .source_type(result.source_type.unwrap_or_default().parse().unwrap())
                              .created_at(result.created_at)
                              .updated_at(result.updated_at)
                              .build()
                          })
                          .collect::<Vec<_>>()
                      })
                    }
                  );

                  let media = Media::builder()
                    .id(UUID4::from(media.id))
                    .name(media.name.clone())
                    .media_type(media.media_type.parse().unwrap())
                    .slug(Slug::from(media.slug.clone()))
                    .release_date(media.release_date)
                    .cover(media.cover.clone())
                    .parental_rating(media.parental_rating as u8)
                    .is_single(media.is_single)
                    .created_at(media.created_at)
                    .updated_at(media.updated_at)
                    .build();
                  Ok(MediaAggregate {
                    media,
                    composers: composers?,
                    interpreters: interpreters?,
                    sources: sources?,
                  })
                    as Result<MediaAggregate, crate::domain::media::repository::Error>
                }
              })
              .collect::<Vec<_>>()
          })
      }
    );

    let items = futures::future::try_join_all(items?).await?;

    Ok(Paged::from_tuple((count?, items), query.page.clone()))
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use sqlx::Postgres;

  use crate::{
    domain::{
      album::{repository::AlbumRepository, Album},
      artist::{repository::ArtistRepository, Artist},
      genre::{Genre, GenreRepository},
      media::{repository::MediaRepository, Media},
    },
    infra::sqlx::{album_repository, artist_repository, genre_repository},
    shared::vo::{Slug, UUID4},
    AppState,
  };

  use super::*;

  #[tokio::test]
  async fn test_create_media() {
    const MEDIA_ID: &str = "99d76707-fcc4-4e6c-be87-c8eddd90ee8d";
    const COMPOSER_ID: &str = "43f3b4e2-a340-4cc0-9eac-9cf2247530c4";
    const INTERPRETER_ID: &str = "bbbdcca6-332e-4534-9832-202dad2798c3";
    const GENRE_ID: &str = "4cc5b396-a40b-4879-9f8e-6934eaa891ac";
    const ALBUM_ID: &str = "dcbddb88-2f85-4561-aca4-b71f517e6643";

    // Load .env file
    dotenvy::dotenv().ok();

    let app_state = AppState::default().await;
    let mut artist_repository = artist_repository::SqlxArtistRepository::new(&app_state);
    let mut album_repository = album_repository::SqlxAlbumRepository::new(&app_state);
    let mut media_repository = SqlxMediaRepository::new(&app_state);
    let mut genre_repository = genre_repository::SqlxGenreRepository::new(&app_state);

    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
        DELETE FROM "media" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(MEDIA_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela media");

      sqlx::query!(
        r#"
        DELETE FROM "artist" WHERE "id" in ($1, $2)
      "#,
        uuid::Uuid::parse_str(COMPOSER_ID).unwrap(),
        uuid::Uuid::parse_str(INTERPRETER_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");

      sqlx::query!(
        r#"
        DELETE FROM "album" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
        DELETE FROM "genre" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(GENRE_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela genre");
    }

    let composer = Artist::builder()
      .id(UUID4::new(COMPOSER_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_create_media_composer").unwrap())
      .build();

    let interpreter = Artist::builder()
      .id(UUID4::new(INTERPRETER_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_create_media_interpreter").unwrap())
      .build();

    let genre = Genre::builder()
      .id(UUID4::new(GENRE_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_create_media_genre").unwrap())
      .build();

    let album = Album::builder()
      .id(UUID4::new(ALBUM_ID).unwrap())
      .name(String::from("Teste"))
      .build();

    let media = Media::builder()
      .id(UUID4::new(MEDIA_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_create_media_media").unwrap())
      .album_ids(HashSet::from([album.id().clone()]))
      .composer_ids(HashSet::from([composer.id().clone()]))
      .interpreter_ids(HashSet::from([interpreter.id().clone()]))
      .genre_ids(HashSet::from([genre.id().clone()]))
      .build();
    cleanup(&app_state.db).await;
    artist_repository
      .create(&composer)
      .await
      .expect("Erro ao criar artista compisitor");
    artist_repository
      .create(&interpreter)
      .await
      .expect("Erro ao criar artista interpreter");
    genre_repository
      .create(&genre)
      .await
      .expect("Erro ao criar genero");
    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    media_repository
      .create(&media)
      .await
      .expect("Erro ao criar media");

    let result = media_repository
      .find_by_id(media.id())
      .await
      .expect("Erro ao buscar media")
      .expect("Media não encontrada");

    cleanup(&app_state.db).await;

    assert_eq!(
      result, media,
      "A media retornada é diferente da media criada"
    );
  }

  #[tokio::test]
  async fn test_update_media() {
    const MEDIA_ID: &str = "bd5d8140-a897-4577-8eb1-d8e6bc8b31bb";
    const COMPOSER_ID: &str = "29be307b-6b7d-487e-a649-d314b408e818";
    const INTERPRETER_ID: &str = "bb79f430-8ed0-4086-b9e7-77bebd0cd3c8";
    const GENRE_ID: &str = "a8029aed-f392-41e6-9aa1-baa531df245a";
    const ALBUM_ID: &str = "68749179-30d0-43c9-baff-870b9ff8255d";

    // Load .env file
    dotenvy::dotenv().ok();
    let app_state = AppState::default().await;
    let mut artist_repository = artist_repository::SqlxArtistRepository::new(&app_state);
    let mut album_repository = album_repository::SqlxAlbumRepository::new(&app_state);
    let mut media_repository = SqlxMediaRepository::new(&app_state);
    let mut genre_repository = genre_repository::SqlxGenreRepository::new(&app_state);
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
        DELETE FROM "media" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(MEDIA_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela media");

      sqlx::query!(
        r#"
        DELETE FROM "artist" WHERE "id" in ($1, $2)
      "#,
        uuid::Uuid::parse_str(COMPOSER_ID).unwrap(),
        uuid::Uuid::parse_str(INTERPRETER_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela artist");

      sqlx::query!(
        r#"
        DELETE FROM "album" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(ALBUM_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela album");

      sqlx::query!(
        r#"
        DELETE FROM "genre" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(GENRE_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela genre");
    }

    let composer = Artist::builder()
      .id(UUID4::new(COMPOSER_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_update_media_composer").unwrap())
      .build();

    let interpreter = Artist::builder()
      .id(UUID4::new(INTERPRETER_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_update_media_interpreter").unwrap())
      .build();

    let genre = Genre::builder()
      .id(UUID4::new(GENRE_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_update_media_genre").unwrap())
      .build();

    let album = Album::builder()
      .id(UUID4::new(ALBUM_ID).unwrap())
      .name(String::from("Teste"))
      .build();

    let mut media = Media::builder()
      .id(UUID4::new(MEDIA_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_update_media_media").unwrap())
      .album_ids(HashSet::from([album.id().clone()]))
      .composer_ids(HashSet::from([composer.id().clone()]))
      .interpreter_ids(HashSet::from([interpreter.id().clone()]))
      .genre_ids(HashSet::from([genre.id().clone()]))
      .build();

    cleanup(&app_state.db).await;

    artist_repository
      .create(&composer)
      .await
      .expect("Erro ao criar artista compisitor");

    artist_repository
      .create(&interpreter)
      .await
      .expect("Erro ao criar artista interpreter");
    genre_repository
      .create(&genre)
      .await
      .expect("Erro ao criar genero");
    album_repository
      .create(&album)
      .await
      .expect("Erro ao criar album");

    media_repository
      .create(&media)
      .await
      .expect("Erro ao criar media");

    media.set_name(String::from("Teste 2"));
    media.set_slug(Slug::new("test_update_media_media_2").unwrap());

    media_repository
      .update(&media)
      .await
      .expect("Erro ao atualizar media");

    let result = media_repository
      .find_by_id(media.id())
      .await
      .expect("Erro ao buscar media")
      .expect("Media não encontrada");

    cleanup(&app_state.db).await;

    assert_eq!(
      result, media,
      "A media retornada é diferente da media atualizada"
    );
  }
}
