use crate::errors::ErrorResponse;
use crate::genre::presenter::{GenreAggregatePresenter, GenrePresenter};
use crate::page::PageParams;
use crate::{app_state::AppState, di};
use actix_web::get;
use actix_web::web::Query;

use super::dto::FindAllQuery;

mod contribute;

#[get("/genres/{slug}")]
async fn get_genre_by_slug(
  app_state: actix_web::web::Data<AppState>,
  path: actix_web::web::Path<shared::vo::Slug>,
) -> impl actix_web::Responder {
  let slug = path.into_inner();

  let mut genre_repository = di::genre::genre_repository(&app_state);

  match application::genre::stories::find_by_slug::execute(&mut genre_repository, slug).await {
    Ok(genre) => actix_web::HttpResponse::Ok().json(GenreAggregatePresenter::from(genre)),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

#[get("/genres")]
async fn find_all(
  app_state: actix_web::web::Data<AppState>,
  Query(page): Query<PageParams>,
  Query(filter): Query<FindAllQuery>,
) -> impl actix_web::Responder {
  let mut genre_repository = di::genre::genre_repository(&app_state);
  match application::genre::stories::find_all::execute(
    &mut genre_repository,
    application::genre::stories::find_all::Input {
      page: page.into(),
      parent_id: filter.parent_id,
      search: filter.search,
    },
  )
  .await
  {
    Ok(result) => actix_web::HttpResponse::Ok().json(result.present::<GenrePresenter>()),
    Err(err) => ErrorResponse::from(err).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config.service(get_genre_by_slug).service(find_all);
  contribute::configure(config);
}
