use activity::presenters::PublicActivityPresenter;
use actix_web::{patch, post};
use errors::ErrorResponse;
use shared::vo::UUID4;
use utils::get_actor;
use web::{Data, Json, Path, ServiceConfig};

use crate::*;

#[patch("/activities/{activity_id}/approve")]
async fn aprove_activity(
  app_state: Data<AppState>,
  path: Path<UUID4>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  let activity_id = path.into_inner();

  let mut activity_repository = di::activity::activity_repository(&app_state);
  let mut user_repository = di::user::user_repository(&app_state);
  let mut genre_repository = di::genre::genre_repository(&app_state);
  let mut artist_repository = di::artist::artist_repository(&app_state);
  let mut album_reposirtory = di::album::album_repository(&app_state);
  let mut media_repository = di::media::media_repository(&app_state);
  let mut source_repository = di::source::source_repository(&app_state);

  let actor = get_actor(&mut user_repository, &session).await;

  if let Err(error) = actor {
    return error.into();
  }

  let actor = actor.unwrap();
  let result = application::activity::stories::approve::execute(
    &mut activity_repository,
    &mut genre_repository,
    &mut artist_repository,
    &mut album_reposirtory,
    &mut media_repository,
    &mut source_repository,
    application::activity::stories::approve::Input { activity_id, actor },
  )
  .await;

  match result {
    Ok(data) => actix_web::HttpResponse::Ok().json(PublicActivityPresenter::from(data)),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

#[patch("/activities/{activity_id}/reject")]
async fn reject_activity(
  state: Data<AppState>,
  path: Path<UUID4>,
  Json(data): Json<super::dto::RejectActivityDto>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  let activity_id = path.into_inner();

  let mut activity_repository = di::activity::activity_repository(&state);
  let mut user_repository = di::user::user_repository(&state);
  let actor = get_actor(&mut user_repository, &session).await;

  if let Err(error) = actor {
    return error.into();
  }

  let actor = actor.unwrap();

  let result = application::activity::stories::reject::execute(
    &mut activity_repository,
    application::activity::stories::reject::Input {
      activity_id,
      reason: data.reason.clone(),
      user: actor,
    },
  )
  .await;

  match result {
    Ok(data) => actix_web::HttpResponse::Ok().json(PublicActivityPresenter::from(data)),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

pub fn configure(config: &mut ServiceConfig) {
  config.service(aprove_activity).service(reject_activity);
}
