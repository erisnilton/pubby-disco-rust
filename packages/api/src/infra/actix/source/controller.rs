use crate::*;

async fn create_source_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: domain::source::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, infra::actix::errors::ErrorResponse> {
  let actor_id = infra::actix::utils::get_actor_id(&session)?;

  let mut source_repository = di::source::repositories::SourceRepository::new(&app_state);
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);

  let activity = domain::source::stories::contribute::execute(
    &mut source_repository,
    &mut activity_repository,
    domain::source::stories::contribute::Input {
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

async fn create_source(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::CreateSourceDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_source_activity(
    app_state,
    session,
    domain::source::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn delete_source(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let source_id = path.into_inner();

  match create_source_activity(
    app_state,
    session,
    domain::source::stories::contribute::ChangeInput::Delete(source_id),
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
      "/contribute/source",
      actix_web::web::post().to(create_source),
    )
    .route(
      "/contribute/source/{genre_id}",
      actix_web::web::delete().to(delete_source),
    );
}
