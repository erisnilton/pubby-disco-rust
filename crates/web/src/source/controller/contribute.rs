use actix_web::{
  delete, post,
  web::{Data, Json, Path, ServiceConfig},
};
use shared::vo::UUID4;

use crate::{
  activity::presenters::PublicActivityPresenter, app_state::AppState, di, errors::ErrorResponse,
  source::dto::CreateSourceDTO, utils::get_actor_id,
};

async fn create_source_activity(
  app_state: Data<AppState>,
  session: actix_session::Session,
  changes: application::source::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, ErrorResponse> {
  let actor_id = get_actor_id(&session)?;

  let mut source_repository = di::source::source_repository(&app_state);
  let mut activity_repository = di::activity::activity_repository(&app_state);

  let activity = application::source::stories::contribute::execute(
    &mut source_repository,
    &mut activity_repository,
    application::source::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(ErrorResponse::from)?;

  Ok(actix_web::HttpResponse::Created().json(PublicActivityPresenter::from(activity)))
}

#[post("/contribute/source")]
async fn create_source(
  app_state: Data<AppState>,
  Json(data): Json<CreateSourceDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_source_activity(
    app_state,
    session,
    application::source::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

#[delete("/contribute/source/{source_id}")]
async fn delete_source(
  app_state: Data<AppState>,
  session: actix_session::Session,
  path: Path<UUID4>,
) -> impl actix_web::Responder {
  let source_id = path.into_inner();

  match create_source_activity(
    app_state,
    session,
    application::source::stories::contribute::ChangeInput::Delete(source_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut ServiceConfig) {
  config.service(create_source).service(delete_source);
}
