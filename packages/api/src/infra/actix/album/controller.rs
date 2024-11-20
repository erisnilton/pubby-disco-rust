use crate::*;

async fn create_album_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: domain::album::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, infra::actix::errors::ErrorResponse> {
  let actor_id = infra::actix::utils::get_actor_id(&session)?;

  let mut album_repository = di::album::repositories::AlbumRepository::new(&app_state);
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);

  let activity = domain::album::stories::contribute::execute(
    &mut album_repository,
    &mut activity_repository,
    domain::album::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(Into::<infra::actix::errors::ErrorResponse>::into)?;

  Ok(actix_web::HttpResponse::Created().json(Into::<
    infra::actix::activity::presenters::PublicActivityPresenter,
  >::into(activity)))
}

async fn create_album(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::CreateAlbumDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_album_activity(
    app_state,
    session,
    domain::album::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn update_album(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::UpdateAlbumDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let album_id = path.into_inner();

  match create_album_activity(
    app_state,
    session,
    domain::album::stories::contribute::ChangeInput::Update {
      id: album_id,
      changes: (data.into()),
    },
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn delete_album(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let album_id = path.into_inner();

  match create_album_activity(
    app_state,
    session,
    domain::album::stories::contribute::ChangeInput::Delete(album_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("/contribute/album", actix_web::web::post().to(create_album))
    .route(
      "/contribute/album/{album_id}",
      actix_web::web::patch().to(update_album),
    )
    .route(
      "/contribute/album/{album_id}",
      actix_web::web::delete().to(delete_album),
    );
}