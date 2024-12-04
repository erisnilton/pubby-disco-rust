use std::collections::HashSet;

use actix_web::web;
use infra::actix::shared::PageParams;

use crate::*;

async fn create_media_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: domain::media::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, infra::actix::errors::ErrorResponse> {
  let actor_id = infra::actix::utils::get_actor_id(&session)?;

  let mut media_repository = di::media::repositories::MediaRepository::new(&app_state);
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);

  let activity = domain::media::stories::contribute::execute(
    &mut media_repository,
    &mut activity_repository,
    domain::media::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(infra::actix::errors::ErrorResponse::from)?;

  Ok(
    actix_web::HttpResponse::Created()
      .json(infra::actix::activity::presenters::PublicActivityPresenter::from(activity)),
  )
}

async fn create_media(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::CreateMediaDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_media_activity(
    app_state,
    session,
    domain::media::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn update_media(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::UpdateMediaDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let media_id = path.into_inner();

  match create_media_activity(
    app_state,
    session,
    domain::media::stories::contribute::ChangeInput::Update {
      id: media_id,
      changes: (data.into()),
    },
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn delete_media(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_media_activity(
    app_state,
    session,
    domain::media::stories::contribute::ChangeInput::Delete(artist_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn find_by(
  app_state: actix_web::web::Data<AppState>,
  web::Query(page): web::Query<PageParams>,
  web::Query(filter): web::Query<super::dto::FindByQuery>,
) -> impl actix_web::Responder {
  let mut media_repository = di::media::repositories::MediaRepository::new(&app_state);
  match domain::media::stories::find_by::execute(
    &mut media_repository,
    domain::media::stories::find_by::Input {
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
    Ok(media) => actix_web::HttpResponse::Ok().json(crate::shared::paged::Paged::<
      infra::actix::media::presenter::MediaAggregatePresenter,
    >::from(media)),
    Err(error) => infra::actix::errors::ErrorResponse::from(error).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("/contribute/media", actix_web::web::post().to(create_media))
    .route("/medias", actix_web::web::get().to(find_by))
    .route(
      "/contribute/media/{media_id}",
      actix_web::web::patch().to(update_media),
    )
    .route(
      "/contribute/media/{media_id}",
      actix_web::web::delete().to(delete_media),
    );
}
