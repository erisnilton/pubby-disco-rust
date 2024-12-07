use ::sqlx;
use application::genre::repository::{FindAllQuery, GenreRepository};
use domain::genre::{aggregate::GenreAggregate, entity::Genre};
use shared::{
  paged::{PageQueryParams, Paged},
  vo::{Slug, UUID4},
};

pub struct SqlxGenreRepository {
  db: sqlx::Pool<sqlx::Postgres>,
}

impl SqlxGenreRepository {
  pub fn new(db: sqlx::PgPool) -> Self {
    Self { db }
  }
}

impl GenreRepository for SqlxGenreRepository {
  async fn find_by_id(
    &mut self,
    id: &UUID4,
  ) -> Result<Option<Genre>, application::genre::repository::Error> {
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
    .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))?;

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

  async fn create(&mut self, genre: &Genre) -> Result<(), application::genre::repository::Error> {
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
    .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))?;
    Ok(())
  }

  async fn update(&mut self, genre: &Genre) -> Result<(), application::genre::repository::Error> {
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
    .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))?;

    Ok(())
  }

  async fn delete_by_id(
    &mut self,
    id: &UUID4,
  ) -> Result<(), application::genre::repository::Error> {
    sqlx::query!(
      r#"DELETE FROM "genre" WHERE "id" = $1"#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .execute(&self.db)
    .await
    .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))?;

    Ok(())
  }

  async fn find_genre_and_subgenre_by_slug(
    &mut self,
    slug: &Slug,
  ) -> Result<Option<GenreAggregate>, application::genre::repository::Error> {
    let result = sqlx::query!(
      r#"
        WITH RECURSIVE "sub_genre" AS (
          SELECT
              "g"."id",
              "g"."name",
              "g"."parent_id",
              "g"."slug",
              "g"."created_at",
              "g"."updated_at"
          FROM
              genre "g"
          WHERE
              "g"."slug" = $1
          UNION ALL
          SELECT
              "g"."id",
              "g"."name",
              "g"."parent_id",
              "g"."slug",
              "g"."created_at",
              "g"."updated_at"
          FROM
              genre "g"
          JOIN "sub_genre" "sg" ON
              "g"."parent_id" = "sg"."id"
        )
        SELECT * FROM "sub_genre";
    "#,
      slug.to_string(),
    )
    .fetch_all(&self.db)
    .await
    .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))?;

    if let Some((genre_row, related_rows)) = result.split_first() {
      return Ok(Some(GenreAggregate::new(
        Genre::builder()
          .id(UUID4::from(genre_row.id.unwrap()))
          .name(genre_row.name.clone().unwrap())
          .slug(Slug::new(genre_row.slug.clone().unwrap()).unwrap())
          .parent_id(genre_row.parent_id.map(UUID4::from))
          .created_at(genre_row.created_at.unwrap())
          .updated_at(genre_row.updated_at.unwrap())
          .build(),
        related_rows
          .iter()
          .map(|row| {
            Genre::builder()
              .id(UUID4::from(row.id.unwrap()))
              .name(row.name.clone().unwrap())
              .slug(Slug::new(row.slug.clone().unwrap()).unwrap())
              .parent_id(row.parent_id.map(UUID4::from))
              .created_at(row.created_at.unwrap())
              .updated_at(row.updated_at.unwrap())
              .build()
          })
          .collect(),
      )));
    }

    Ok(None)
  }

  async fn find_all(
    &mut self,
    query: &FindAllQuery,
  ) -> Result<Paged<Genre>, application::genre::repository::Error> {
    let (count, items) = tokio::join!(
      async {
        let mut builder =
          sqlx::QueryBuilder::new(r#"SELECT COUNT("id") FROM "genre" WHERE "parent_id" "#);

        if let Some(parent_id) = &query.parent_id {
          builder
            .push(r#"="#)
            .push_bind(uuid::Uuid::from(parent_id.clone()));
        } else {
          builder.push(r#"IS NULL"#);
        }
        if let Some(search) = &query.search {
          builder
            .push(r#" AND ( "name" ILIKE '%' || "#)
            .push_bind(search)
            .push(r#" || '%' OR "slug" ILIKE '%' || "#)
            .push_bind(search)
            .push(r#" || '%')"#);
        }

        builder
          .build_query_scalar::<Option<i64>>()
          .fetch_one(&self.db)
          .await
          .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))
          .map(|result| result.unwrap_or(0) as usize)
      },
      async {
        #[derive(sqlx::FromRow)]
        struct Record {
          id: uuid::Uuid,
          name: String,
          slug: String,
          parent_id: Option<uuid::Uuid>,
          created_at: chrono::NaiveDateTime,
          updated_at: chrono::NaiveDateTime,
        }
        let page_params = PageQueryParams::from(query.page.clone());

        let mut builder = sqlx::QueryBuilder::new(
          r#"SELECT "id", "name", "slug", "parent_id", "created_at", "updated_at" FROM "genre" WHERE "parent_id" "#,
        );

        if let Some(parent_id) = &query.parent_id {
          builder
            .push(" = ")
            .push_bind(uuid::Uuid::from(parent_id.clone()));
        } else {
          builder.push(r#"IS NULL"#);
        }

        if let Some(search) = &query.search {
          builder
            .push(r#" AND ( "name" ILIKE '%' || "#)
            .push_bind(search)
            .push(r#" || '%' OR "slug" ILIKE '%' || "#)
            .push_bind(search)
            .push(r#" || '%')"#);
        }

        builder
          .push(" LIMIT ")
          .push_bind(page_params.take as i32)
          .push(" OFFSET ")
          .push_bind(page_params.skip as i32);

        builder
          .build_query_as::<Record>()
          .fetch_all(&self.db)
          .await
          .map_err(|err| application::genre::repository::Error::DatabaseError(err.to_string()))
          .map(|result| {
            result
              .into_iter()
              .map(|row| {
                Genre::builder()
                  .id(UUID4::from(row.id))
                  .name(row.name)
                  .parent_id(row.parent_id.map(UUID4::from))
                  .slug(Slug::from(row.slug))
                  .created_at(row.created_at)
                  .updated_at(row.updated_at)
                  .build()
              })
              .collect::<Vec<Genre>>()
          })
      }
    );

    Ok(Paged::from_tuple((count?, items?), query.page.clone()))
  }
}

#[cfg(test)]
mod tests {
  // use super::*;
  // use crate::{domain::genre::GenreBuilder, shared::vo::Slug, AppState};

  // #[tokio::test]
  // async fn test_create() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const GENRE_ID: &str = "698db80e-6d0e-4768-8a55-f9ff966f2523";

  //   async fn clean(db: &sqlx::pool::Pool<sqlx::Postgres>) {
  //     sqlx::query!(
  //       r#"DELETE FROM "genre" WHERE "id" = $1"#,
  //       uuid::Uuid::parse_str(GENRE_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Error cleaning database");
  //   }

  //   let app_state = AppState::default().await;

  //   let mut repository_genre = SqlxGenreRepository::new(&app_state);

  //   let genre = Genre::builder()
  //     .id(UUID4::new(GENRE_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(Slug::new("test_create_genre").unwrap())
  //     .build();

  //   clean(&app_state.db).await;

  //   repository_genre
  //     .create(&genre)
  //     .await
  //     .expect("Falha ao criar Gênero");

  //   let result = repository_genre
  //     .find_by_id(genre.id())
  //     .await
  //     .expect("Falha ao buscar Gênero")
  //     .expect("Gênero não encontrado");

  //   clean(&app_state.db).await;

  //   assert_eq!(result, genre, "Gênero não é igual ao esperado");
  // }

  // #[tokio::test]
  // async fn test_update() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const GENRE_ID: &str = "5c86933d-8e5c-4790-b692-f0562f788322";

  //   async fn clean(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       r#"DELETE FROM "genre" WHERE "id" = $1"#,
  //       uuid::Uuid::parse_str(GENRE_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao excluir dados de teste");
  //   }

  //   let app_state = AppState::default().await;

  //   let mut repository_genre = SqlxGenreRepository::new(&app_state);

  //   let genre = Genre::builder()
  //     .id(UUID4::new(GENRE_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(Slug::new("test_update_genre").unwrap())
  //     .build();

  //   clean(&app_state.db).await;

  //   repository_genre.create(&genre).await.unwrap();

  //   let genre = GenreBuilder::from(genre)
  //     .name(String::from("Test 2"))
  //     .slug(Slug::new("test_update_genre2").unwrap())
  //     .build();

  //   repository_genre
  //     .update(&genre)
  //     .await
  //     .expect("Falha ao atualizar Gênero");

  //   let result = repository_genre
  //     .find_by_id(genre.id())
  //     .await
  //     .expect("Falha ao buscar Gênero")
  //     .expect("Gênero não encontrado");

  //   clean(&app_state.db).await;

  //   assert_eq!(result, genre, "Gênero não é igual ao esperado");
  // }

  // #[tokio::test]
  // async fn test_delete() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();

  //   const GENRE_ID: &str = "0dd24433-d16e-4b36-8f7e-27ce91c81f09";

  //   async fn clean(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       r#"DELETE FROM "genre" WHERE "id" = $1"#,
  //       uuid::Uuid::parse_str(GENRE_ID).unwrap()
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao excluir dados de teste");
  //   }

  //   let app_state = AppState::default().await;

  //   let mut repository_genre = SqlxGenreRepository::new(&app_state);

  //   let genre = Genre::builder()
  //     .id(UUID4::new(GENRE_ID).unwrap())
  //     .name(String::from("Test"))
  //     .slug(Slug::new("test_delete_genre").unwrap())
  //     .build();

  //   clean(&app_state.db).await;

  //   repository_genre
  //     .create(&genre)
  //     .await
  //     .expect("Falha ao criar Gênero");

  //   repository_genre
  //     .delete_by_id(genre.id())
  //     .await
  //     .expect("Falha ao deletar Gênero");

  //   let result = repository_genre
  //     .find_by_id(genre.id())
  //     .await
  //     .expect("Falha ao buscar Gênero");

  //   assert_eq!(result, None, "Gênero não foi excluído");
  // }

  // #[tokio::test]
  // async fn test_find_genre_and_subgenre_by_id() {
  //   // Load .env file
  //   dotenvy::dotenv().ok();
  //   let app_state = AppState::default().await;
  //   let mut genre_repository = SqlxGenreRepository::new(&app_state);

  //   const GENRE_TEST_1_ID: &str = "2cf76ffd-910b-4081-9a05-e50b46598870";
  //   const GENRE_TEST_2_ID: &str = "fc19218b-b0d7-49cf-a66b-2b068678b62d";
  //   const GENRE_TEST_3_ID: &str = "892fd6e7-3b5c-4e41-9d01-89474f834edf";

  //   async fn clean(db: &sqlx::PgPool) {
  //     sqlx::query!(
  //       r#"DELETE FROM "genre" WHERE "id" IN ($1, $2, $3)"#,
  //       uuid::Uuid::parse_str(GENRE_TEST_1_ID).unwrap(),
  //       uuid::Uuid::parse_str(GENRE_TEST_2_ID).unwrap(),
  //       uuid::Uuid::parse_str(GENRE_TEST_3_ID).unwrap(),
  //     )
  //     .execute(db)
  //     .await
  //     .expect("Falha ao excluir dados de teste");
  //   }

  //   let genre_1 = Genre::builder()
  //     .id(UUID4::new(GENRE_TEST_1_ID).unwrap())
  //     .name(String::from("Test 1"))
  //     .slug(Slug::new("test_find_genre_and_subgenre_by_id_1").unwrap())
  //     .build();

  //   let genre_2 = Genre::builder()
  //     .id(UUID4::new(GENRE_TEST_2_ID).unwrap())
  //     .name(String::from("Test 2"))
  //     .slug(Slug::new("test_find_genre_and_subgenre_by_id_2").unwrap())
  //     .parent_id(Some(genre_1.id().clone()))
  //     .build();

  //   let genre_3 = Genre::builder()
  //     .id(UUID4::new(GENRE_TEST_3_ID).unwrap())
  //     .name(String::from("Test 3"))
  //     .slug(Slug::new("test_find_genre_and_subgenre_by_id_3").unwrap())
  //     .parent_id(Some(genre_2.id().clone()))
  //     .build();

  //   clean(&app_state.db).await;

  //   genre_repository
  //     .create(&genre_1)
  //     .await
  //     .expect("Falha ao criar Gênero 1");

  //   genre_repository
  //     .create(&genre_2)
  //     .await
  //     .expect("Falha ao criar Gênero 2");

  //   genre_repository
  //     .create(&genre_3)
  //     .await
  //     .expect("Falha ao criar Gênero 3");

  //   let result = genre_repository
  //     .find_genre_and_subgenre_by_slug(genre_1.slug())
  //     .await
  //     .expect("Falha ao buscar Gênero e Subgêneros")
  //     .expect("Gênero e Subgêneros não encontrados");

  //   clean(&app_state.db).await;

  //   assert_eq!(
  //     GenreAggregate::new(genre_1, vec![genre_2, genre_3]),
  //     result,
  //     "Gênero e Subgêneros não são iguais ao esperado"
  //   );
  // }
}
