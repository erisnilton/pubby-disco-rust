use std::collections::HashSet;

use actix_web::{get, web::Query};

use crate::media::presenter::MediaAggregatePresenter;
use crate::{app_state::AppState, di, errors::ErrorResponse, page::PageParams};

mod contribute;

#[get("/media")]
async fn find_by(
  app_state: actix_web::web::Data<AppState>,
  Query(page): Query<PageParams>,
  Query(filter): Query<super::dto::FindByQuery>,
) -> impl actix_web::Responder {
  let mut media_repository = di::media::media_repository(&app_state);
  match application::media::stories::find_by::execute(
    &mut media_repository,
    application::media::stories::find_by::Input {
      page: page.into(),
      search: filter.search,
      release_date: filter.release_date,
      min_release_date: filter.min_release_date,
      max_release_date: filter.max_release_date,
      parental_rating: filter.parental_rating,
      min_parental_rating: filter.min_parental_rating,
      max_parental_rating: filter.max_parental_rating,
      is_single: filter.is_single,
      media_type: filter.media_type,
      slug: filter.slug,
      artist_ids: filter.artist_id.map(|id| HashSet::from([id.clone()])),
      composer_ids: filter.composer_id.map(|id| HashSet::from([id.clone()])),
      genre_ids: filter.genre_id.map(|id| HashSet::from([id.clone()])),
      album_ids: filter.album_id.map(|id| HashSet::from([id.clone()])),
    },
  )
  .await
  {
    Ok(result) => actix_web::HttpResponse::Ok().json(result.present::<MediaAggregatePresenter>()),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config.service(find_by);

  contribute::configure(config);
}
