use crate::*;

async fn create_artist_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: domain::artist::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, infra::actix::errors::ErrorResponse> {
  let actor_id = infra::actix::utils::get_actor_id(&session)?;

  let mut artist_repository = di::artist::repositories::ArtistRepository::new(&app_state);
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);

  let activity = domain::artist::stories::contribute::execute(
    &mut artist_repository,
    &mut activity_repository,
    domain::artist::stories::contribute::Input {
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

async fn create_artist(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::CreateArtistDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_artist_activity(
    app_state,
    session,
    domain::artist::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn update_artist(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::UpdateArtistDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_artist_activity(
    app_state,
    session,
    domain::artist::stories::contribute::ChangeInput::Update {
      id: artist_id,
      changes: (data.into()),
    },
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn delete_artist(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_artist_activity(
    app_state,
    session,
    domain::artist::stories::contribute::ChangeInput::Delete(artist_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route(
      "/contribute/artist",
      actix_web::web::post().to(create_artist),
    )
    .route(
      "/contribute/artist/{artist_id}",
      actix_web::web::patch().to(update_artist),
    )
    .route(
      "/contribute/artist/{artist_id}",
      actix_web::web::delete().to(delete_artist),
    );
}
