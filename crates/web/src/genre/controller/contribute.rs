use actix_web::{delete, patch, post};

use crate::{
  activity::presenters::PublicActivityPresenter,
  app_state::AppState,
  di,
  errors::ErrorResponse,
  genre::dto::{CreateGenreDTO, UpdateGenreDto},
  utils::get_actor_id,
};

async fn create_genre_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: application::genre::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, ErrorResponse> {
  let actor_id = get_actor_id(&session)?;

  let mut genre_repository = di::genre::genre_repository(&app_state);
  let mut activity_repository = di::activity::activity_repository(&app_state);

  let activity = application::genre::stories::contribute::execute(
    &mut genre_repository,
    &mut activity_repository,
    application::genre::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(ErrorResponse::from)?;

  Ok(actix_web::HttpResponse::Created().json(PublicActivityPresenter::from(activity)))
}

#[post("/contribute/genre")]
async fn create_genre(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<CreateGenreDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_genre_activity(
    app_state,
    session,
    application::genre::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

#[patch("/contribute/genre/{genre_id}")]
async fn update_genre(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<UpdateGenreDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let genre_id = path.into_inner();

  match create_genre_activity(
    app_state,
    session,
    application::genre::stories::contribute::ChangeInput::Update {
      id: genre_id,
      changes: (data.into()),
    },
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

#[delete("/contribute/genre/{genre_id}")]
async fn delete_genre(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let genre_id = path.into_inner();

  match create_genre_activity(
    app_state,
    session,
    application::genre::stories::contribute::ChangeInput::Delete(genre_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .service(create_genre)
    .service(update_genre)
    .service(delete_genre);
}
