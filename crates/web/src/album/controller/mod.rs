use actix_web::get;
use actix_web::web::{Query, ServiceConfig};

use crate::app_state::AppState;
use crate::di;
use crate::errors::ErrorResponse;
use crate::page::PageParams;

use super::presenter::AlbumAggregatePresenter;

mod contribute;

#[get("/albums/{artist_slug}/{slug}")]
async fn find_album_slug(
  app_state: actix_web::web::Data<AppState>,
  path: actix_web::web::Path<(shared::vo::Slug, shared::vo::Slug)>,
) -> impl actix_web::Responder {
  let (artist_slug, slug) = path.into_inner();

  let mut album_repository = di::album::album_repository(&app_state);

  match application::album::stories::find_by_slug::execute(
    &mut album_repository,
    &slug,
    &artist_slug,
  )
  .await
  {
    Ok(aggregate) => actix_web::HttpResponse::Ok().json(AlbumAggregatePresenter::from(aggregate)),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

#[get("/albums")]
async fn find_by(
  app_state: actix_web::web::Data<AppState>,
  Query(page): Query<PageParams>,
  Query(filter): Query<super::dto::FindAllQuery>,
) -> impl actix_web::Responder {
  let mut album_repository = di::album::album_repository(&app_state);
  match application::album::stories::find_by::execute(
    &mut album_repository,
    application::album::stories::find_by::Input {
      page: page.into(),
      slug: filter.slug,
      name: filter.name,
      artist_ids: filter.artist_id.map(|id| vec![id]),
      album_type: filter.album_type,
      release_date: filter.release_date,
      min_release_date: filter.min_release_date,
      max_release_date: filter.max_release_date,
      parental_rating: filter.parental_rating,
      min_parental_rating: filter.min_parental_rating,
      max_parental_rating: filter.max_parental_rating,
      search: filter.search,
    },
  )
  .await
  {
    Ok(result) => actix_web::HttpResponse::Ok().json(result.present::<AlbumAggregatePresenter>()),
    Err(err) => ErrorResponse::from(err).into(),
  }
}

pub fn configure(config: &mut ServiceConfig) {
  config.service(find_album_slug).service(find_by);
  contribute::configure(config);
}
