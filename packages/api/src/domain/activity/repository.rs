use std::future::Future;

use crate::shared::vo::UUID4;

use super::Activity;

#[derive(Debug, Clone)]
pub enum Error {
  InternalServerError(String),
  EntityNotFound,
}

pub trait ActivityRepository {
  /**
   * Cria uma nova atividade de contribuição.
   */
  fn create(&mut self, activity: &Activity) -> impl Future<Output = Result<Activity, Error>>;

  /**
   * Busca uma atividade de contribuição pelo seu identificador e retorna a atividade encontrada ou None caso não exista.
   */
  fn find_by_id(&self, id: &UUID4) -> impl Future<Output = Result<Option<Activity>, Error>>;

  /**
   * Atualiza uma atividade de contribuição.
   */
  fn update(&mut self, activity: &Activity) -> impl Future<Output = Result<Activity, Error>>;
}
