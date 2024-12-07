use actix_web::{delete, post};

use crate::{
  activity::presenters::PublicActivityPresenter,
  app_state::AppState,
  di,
  errors::ErrorResponse,
  media::dto::{CreateMediaDTO, UpdateMediaDto},
  utils::get_actor_id,
};

async fn create_media_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: application::media::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, ErrorResponse> {
  let actor_id = get_actor_id(&session)?;

  let mut media_repository = di::media::media_repository(&app_state);
  let mut activity_repository = di::activity::activity_repository(&app_state);

  let activity = application::media::stories::contribute::execute(
    &mut media_repository,
    &mut activity_repository,
    application::media::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(ErrorResponse::from)?;

  Ok(actix_web::HttpResponse::Created().json(PublicActivityPresenter::from(activity)))
}

#[post("/contribute/media")]
async fn create_media(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<CreateMediaDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_media_activity(
    app_state,
    session,
    application::media::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

#[post("/contribute/media/{media_id}")]
async fn update_media(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<UpdateMediaDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let media_id = path.into_inner();

  match create_media_activity(
    app_state,
    session,
    application::media::stories::contribute::ChangeInput::Update {
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

#[delete("/contribute/media/{media_id}")]
async fn delete_media(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_media_activity(
    app_state,
    session,
    application::media::stories::contribute::ChangeInput::Delete(artist_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .service(create_media)
    .service(update_media)
    .service(delete_media);
}
