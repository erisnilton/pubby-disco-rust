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

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("/contribute/media", actix_web::web::post().to(create_media))
    .route(
      "/contribute/media/{media_id}",
      actix_web::web::patch().to(update_media),
    )
    .route(
      "/contribute/media/{media_id}",
      actix_web::web::delete().to(delete_media),
    );
}
