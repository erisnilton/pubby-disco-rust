mod contribute;
use actix_web::{get, web::Query};

use crate::{
  app_state::AppState, artist::dto::FindAllQuery, artist::presenter::ArtistPresenter, di,
  errors::ErrorResponse, page::PageParams,
};

#[get("/artists")]
async fn find_all(
  app_state: actix_web::web::Data<AppState>,
  Query(page): Query<PageParams>,
  Query(filter): Query<FindAllQuery>,
) -> impl actix_web::Responder {
  let mut artist_repository = di::artist::artist_repository(&app_state);

  match application::artist::stories::find_all::execute(
    &mut artist_repository,
    application::artist::stories::find_all::Input {
      page: page.into(),
      search: filter.search,
      country: filter.country,
    },
  )
  .await
  {
    Ok(result) => actix_web::HttpResponse::Ok().json(result.present::<ArtistPresenter>()),
    Err(err) => ErrorResponse::from(err).into(),
  }
}

#[get("/artists/{slug}")]
async fn find_by_slug(
  app_state: actix_web::web::Data<AppState>,
  path: actix_web::web::Path<shared::vo::Slug>,
) -> impl actix_web::Responder {
  let slug = path.into_inner();
  let mut artist_repository = di::artist::artist_repository(&app_state);

  match application::artist::stories::find_by_slug::execute(&mut artist_repository, &slug).await {
    Ok(artist) => actix_web::HttpResponse::Ok().json(ArtistPresenter::from(artist)),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config.service(find_all).service(find_by_slug);

  contribute::configure(config);
}
