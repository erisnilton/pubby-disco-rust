use actix_web::{delete, patch, post};

use crate::{
  activity::presenters::PublicActivityPresenter,
  app_state::AppState,
  artist::dto::{CreateArtistDTO, UpdateArtistDto},
  di,
  errors::ErrorResponse,
  utils::get_actor_id,
};

async fn create_artist_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: application::artist::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, ErrorResponse> {
  let actor_id = get_actor_id(&session)?;

  let mut artist_repository = di::artist::artist_repository(&app_state);
  let mut activity_repository = di::activity::activity_repository(&app_state);

  let activity = application::artist::stories::contribute::execute(
    &mut artist_repository,
    &mut activity_repository,
    application::artist::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(Into::<ErrorResponse>::into)?;

  Ok(actix_web::HttpResponse::Created().json(PublicActivityPresenter::from(activity)))
}

#[post("/contribute/artist")]
async fn create_artist(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<CreateArtistDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_artist_activity(
    app_state,
    session,
    application::artist::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

#[patch("/contribute/artist/{artist_id}")]
async fn update_artist(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<UpdateArtistDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_artist_activity(
    app_state,
    session,
    application::artist::stories::contribute::ChangeInput::Update {
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

#[delete("/contribute/artist/{artist_id}")]
async fn delete_artist(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  println!("DELETE ARTIST {path:?}");
  let artist_id = path.into_inner();

  match create_artist_activity(
    app_state,
    session,
    application::artist::stories::contribute::ChangeInput::Delete(artist_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .service(create_artist)
    .service(update_artist)
    .service(delete_artist);
}
