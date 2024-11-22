pub struct SqlxSourceRepository {
  db: sqlx::Pool<sqlx::Postgres>,
}

impl SqlxSourceRepository {
  pub fn new(app_state: &crate::AppState) -> Self {
    Self {
      db: app_state.db.clone(),
    }
  }
}

fn to_database_error(error: sqlx::error::Error) -> crate::domain::source::repository::Error {
  crate::domain::source::repository::Error::DatabaseError(error.to_string())
}

impl crate::domain::source::repository::SourceRepository for SqlxSourceRepository {
  async fn create(
    &mut self,
    source: &crate::domain::source::source_entity::Source,
  ) -> Result<(), crate::domain::source::repository::Error> {
    sqlx::query!(
      r#"
      INSERT INTO "source" ("id", "source_type", "src", "media_id", "created_at" ,"updated_at")
      VALUES($1,$2,$3,$4,$5,$6)
    "#,
      Into::<uuid::Uuid>::into(source.id().clone()),
      source.source_type().to_string(),
      source.src(),
      Into::<uuid::Uuid>::into(source.media_id().clone()),
      source.created_at(),
      source.updated_at()
    )
    .execute(&self.db)
    .await
    .map_err(to_database_error)?;
    Ok(())
  }

  async fn find_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<
    Option<crate::domain::source::source_entity::Source>,
    crate::domain::source::repository::Error,
  > {
    let source = sqlx::query!(
      r#"
        SELECT "id", "source_type","src", "media_id","created_at", "updated_at" FROM "source" WHERE "id" = $1
      "#,
      Into::<uuid::Uuid>::into(id.clone())
    )
    .fetch_optional(&self.db)
    .await
    .map_err(to_database_error)?
    .map(|value| crate::domain::source::source_entity::Source::builder()
    .id(crate::shared::vo::UUID4::from(value.id))
    .source_type(value.source_type.parse().unwrap())
    .src(value.src)
    .media_id(crate::shared::vo::UUID4::from(value.media_id))
    .created_at(value.created_at)
    .updated_at(value.updated_at)
    .build());

    if source.is_none() {
      return Ok(None);
    }

    Ok(Some(source.unwrap()))
  }

  async fn delete_by_id(
    &mut self,
    id: &crate::shared::vo::UUID4,
  ) -> Result<(), crate::domain::source::repository::Error> {
    sqlx::query!(
      r#"DELETE FROM "source" WHERE "id" = $1"#,
      uuid::Uuid::from(id.clone())
    )
    .execute(&self.db)
    .await
    .map_err(to_database_error)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {

  use sqlx::Postgres;

  use crate::{
    domain::{
      media::{repository::MediaRepository, Media},
      source::repository::SourceRepository,
    },
    shared::vo::{Slug, UUID4},
    AppState,
  };

  #[tokio::test]
  async fn test_create_source() {
    const SOURCE_ID: &str = "856093f9-be6d-4fb0-8f4e-033ad31818b8";
    const MEDIA_ID: &str = "75c6235f-0edd-4563-8f60-81e8049ccbcb";

    // Load .env file
    dotenvy::dotenv().ok();
    let app_state = AppState::default().await;
    let mut media_repository = crate::infra::sqlx::SqlxMediaRepository::new(&app_state);
    let mut source_repository = crate::infra::sqlx::SqlxSourceRepository::new(&app_state);
    async fn cleanup(db: &sqlx::pool::Pool<Postgres>) {
      sqlx::query!(
        r#"
        DELETE FROM "media" WHERE "id" = $1
      "#,
        uuid::Uuid::parse_str(MEDIA_ID).unwrap()
      )
      .execute(db)
      .await
      .expect("Erro ao excluir dados de teste da tabela source");
    }

    let media = Media::builder()
      .id(UUID4::new(MEDIA_ID).unwrap())
      .name(String::from("Teste"))
      .slug(Slug::new("test_create_source_media").unwrap())
      .build();

    let source = crate::domain::source::source_entity::Source::builder()
      .id(UUID4::new(SOURCE_ID).unwrap())
      .source_type(crate::domain::source::source_entity::SourceType::Youtube)
      .src(String::from("https://www.youtube.com/watch?v=VHA2ASr8rmY"))
      .media_id(media.id().clone())
      .build();

    cleanup(&app_state.db).await;

    media_repository
      .create(&media)
      .await
      .expect("Erro ao criar media");

    source_repository
      .create(&source)
      .await
      .expect("Erro ao criar source");

    let result = source_repository
      .find_by_id(source.id())
      .await
      .expect("Erro ao buscar o source")
      .expect("Source não encontrada");

    cleanup(&app_state.db).await;

    assert_eq!(
      result, source,
      "O source retornado é diferente do source criada"
    );
  }
}
