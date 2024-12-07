use actix_web::{
  delete, patch, post,
  web::{Data, Json, Path, ServiceConfig},
};

use crate::{
  activity::presenters::PublicActivityPresenter,
  album::dto::{CreateAlbumDTO, UpdateAlbumDto},
  app_state::AppState,
  di,
  errors::ErrorResponse,
  utils::get_actor_id,
};

async fn create_album_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: application::album::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, ErrorResponse> {
  let actor_id = get_actor_id(&session)?;

  let mut album_repository = di::album::album_repository(&app_state);
  let mut activity_repository = di::activity::activity_repository(&app_state);

  let activity = application::album::stories::contribute::execute(
    &mut album_repository,
    &mut activity_repository,
    application::album::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(ErrorResponse::from)?;

  Ok(actix_web::HttpResponse::Created().json(PublicActivityPresenter::from(activity)))
}

#[post("/contribute/album")]
async fn create_album(
  app_state: Data<AppState>,
  Json(data): Json<CreateAlbumDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_album_activity(
    app_state,
    session,
    application::album::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

#[patch("/contribute/album/{album_id}")]
async fn update_album(
  app_state: Data<AppState>,
  Json(data): Json<UpdateAlbumDto>,
  session: actix_session::Session,
  path: Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let album_id = path.into_inner();

  match create_album_activity(
    app_state,
    session,
    application::album::stories::contribute::ChangeInput::Update {
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

#[delete("/contribute/album/{album_id}")]
async fn delete_album(
  app_state: Data<AppState>,
  session: actix_session::Session,
  path: Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let album_id = path.into_inner();

  match create_album_activity(
    app_state,
    session,
    application::album::stories::contribute::ChangeInput::Delete(album_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut ServiceConfig) {
  config
    .service(create_album)
    .service(update_album)
    .service(delete_album);
}
