use std::collections::HashSet;

use sqlx::PgConnection;

use shared::vo::UUID4;

pub struct ManyToManyManager {
  table_name: String,
  column: String,
  related_column: String,
}

impl ManyToManyManager {
  pub async fn insert<'c, E>(
    &self,
    executor: E,
    id: &UUID4,
    item_id: &UUID4,
  ) -> Result<(), sqlx::Error>
  where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
  {
    sqlx::query(&format!(
      r#"INSERT INTO "{}" ("{}","{}") VALUES ($1, $2)"#,
      self.table_name, self.column, self.related_column
    ))
    .bind(uuid::Uuid::from(id.clone()))
    .bind(uuid::Uuid::from(item_id.clone()))
    .execute(executor)
    .await?;

    Ok(())
  }

  pub async fn insert_many<'c, E>(
    &self,
    executor: E,
    id: &UUID4,
    item_ids: &HashSet<UUID4>,
  ) -> Result<(), sqlx::Error>
  where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
  {
    if item_ids.is_empty() {
      return Ok(());
    }

    let mut sql = format!(
      r#"INSERT INTO "{}" ("{}","{}") VALUES"#,
      self.table_name, self.column, self.related_column
    );

    for i in 1..=item_ids.len() {
      if i > 1 {
        sql.push(',');
      }

      sql.push_str(&format!("($1, ${})", i + 1));
    }

    let mut query = sqlx::query(&sql).bind(uuid::Uuid::from(id.clone()));

    for id in item_ids.iter() {
      query = query.bind(uuid::Uuid::from(id.clone()));
    }

    query.execute(executor).await?;

    Ok(())
  }

  pub async fn delete<'c, E>(
    &self,
    executor: E,
    id: &UUID4,
    item_id: &UUID4,
  ) -> Result<(), sqlx::Error>
  where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
  {
    sqlx::query(&format!(
      r#"DELETE FROM "{}" WHERE "{}" = $1 AND "{}" = $2"#,
      self.table_name, self.column, self.related_column
    ))
    .bind(uuid::Uuid::from(id.clone()))
    .bind(uuid::Uuid::from(item_id.clone()))
    .execute(executor)
    .await?;

    Ok(())
  }

  pub async fn delete_many<'c, E>(
    &self,
    executor: E,
    id: &UUID4,
    item_ids: &HashSet<UUID4>,
  ) -> Result<(), sqlx::Error>
  where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
  {
    if item_ids.is_empty() {
      return Ok(());
    }

    let mut sql = format!(
      r#"DELETE FROM "{}" WHERE "{}" = $1 AND "{}" IN ("#,
      self.table_name, self.column, self.related_column
    );

    for i in 1..=item_ids.len() {
      if i > 1 {
        sql.push_str(", ");
      }

      sql.push_str(&format!("${}", i + 1));
    }

    sql.push(')');

    let mut query = sqlx::query(&sql).bind(uuid::Uuid::from(id.clone()));

    for id in item_ids.iter() {
      query = query.bind(uuid::Uuid::from(id.clone()));
    }

    query.execute(executor).await?;

    Ok(())
  }

  pub async fn find_by<'c, E>(&self, executor: E, id: &UUID4) -> Result<HashSet<UUID4>, sqlx::Error>
  where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
  {
    let result: Vec<uuid::Uuid> = sqlx::query_scalar(&format!(
      r#"SELECT "{}" FROM "{}" WHERE "{}" = $1"#,
      self.related_column, self.table_name, self.column
    ))
    .bind(uuid::Uuid::from(id.clone()))
    .fetch_all(executor)
    .await?;

    Ok(result.into_iter().map(UUID4::from).collect())
  }

  pub async fn update(
    &self,
    executor: &mut PgConnection,
    id: &UUID4,
    item_ids: &HashSet<UUID4>,
  ) -> Result<(), sqlx::Error> {
    let old_ids = self.find_by(&mut *executor, id).await?;

    let (to_delete, to_insert) = {
      let mut to_delete = HashSet::new();
      let mut to_insert = HashSet::new();

      for id in old_ids.iter() {
        if !item_ids.contains(id) {
          to_delete.insert(id.clone());
        }
      }

      for id in item_ids.iter() {
        if !old_ids.contains(id) {
          to_insert.insert(id.clone());
        }
      }

      (to_delete, to_insert)
    };

    self.delete_many(&mut *executor, id, &to_delete).await?;
    self.insert_many(&mut *executor, id, &to_insert).await?;

    Ok(())
  }
}

pub struct ManyToManyBuilder {
  obj: ManyToManyManager,
}

impl Default for ManyToManyBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl ManyToManyBuilder {
  pub fn new() -> Self {
    Self {
      obj: ManyToManyManager {
        table_name: String::new(),
        column: String::new(),
        related_column: String::new(),
      },
    }
  }

  pub fn table(mut self, table: impl Into<String>) -> Self {
    self.obj.table_name = table.into();
    self
  }

  pub fn column(mut self, column: impl Into<String>) -> Self {
    self.obj.column = column.into();
    self
  }

  pub fn related_column(mut self, related_column: impl Into<String>) -> Self {
    self.obj.related_column = related_column.into();
    self
  }

  pub fn build(self) -> ManyToManyManager {
    self.obj
  }
}
