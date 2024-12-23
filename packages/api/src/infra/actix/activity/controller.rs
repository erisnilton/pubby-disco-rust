use shared::vo::UUID4;

use crate::*;

async fn aprove_activity(
  app_state: actix_web::web::Data<AppState>,
  path: actix_web::web::Path<UUID4>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  let activity_id = path.into_inner();
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);
  let mut user_repository = di::user::repositories::UserRepository::new(&app_state);
  let mut genre_repository = di::genre::repositories::GenreRepository::new(&app_state);
  let mut artist_repository = di::artist::repositories::ArtistRepository::new(&app_state);
  let mut album_reposirtory = di::album::repositories::AlbumRepository::new(&app_state);
  let mut media_repository = di::media::repositories::MediaRepository::new(&app_state);
  let mut source_repository = di::source::repositories::SourceRepository::new(&app_state);

  let actor = crate::infra::actix::utils::get_actor(&mut user_repository, &session).await;

  if let Err(error) = actor {
    return error.into();
  }

  let actor = actor.unwrap();
  let result = domain::activity::stories::approve::execute(
    &mut activity_repository,
    &mut genre_repository,
    &mut artist_repository,
    &mut album_reposirtory,
    &mut media_repository,
    &mut source_repository,
    domain::activity::stories::approve::Input { activity_id, actor },
  )
  .await;

  match result {
    Ok(data) => actix_web::HttpResponse::Ok().json(Into::<
      super::presenters::PublicActivityPresenter,
    >::into(data)),
    Err(error) => Into::<infra::actix::errors::ErrorResponse>::into(error).into(),
  }
}

async fn reject_activity(
  state: actix_web::web::Data<AppState>,
  path: actix_web::web::Path<UUID4>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::RejectActivityDto>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  let activity_id = path.into_inner();

  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&state);
  let mut user_repository = di::user::repositories::UserRepository::new(&state);
  let actor = crate::infra::actix::utils::get_actor(&mut user_repository, &session).await;

  if let Err(error) = actor {
    return error.into();
  }

  let actor = actor.unwrap();

  let result = domain::activity::stories::reject::execute(
    &mut activity_repository,
    domain::activity::stories::reject::Input {
      activity_id,
      reason: data.reason.clone(),
      user: actor,
    },
  )
  .await;

  match result {
    Ok(data) => actix_web::HttpResponse::Ok().json(Into::<
      super::presenters::PublicActivityPresenter,
    >::into(data)),
    Err(error) => Into::<infra::actix::errors::ErrorResponse>::into(error).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route(
      "/activities/{activity_id}/reject",
      actix_web::web::patch().to(reject_activity),
    )
    .route(
      "/activities/{activity_id}/approve",
      actix_web::web::patch().to(aprove_activity),
    );
}
