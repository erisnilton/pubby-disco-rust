use crate::*;

async fn create_genre_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: domain::genre::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, infra::actix::errors::ErrorResponse> {
  let actor_id = infra::actix::utils::get_actor_id(&session)?;

  let mut genre_repository = di::genre::repositories::GenreRepository::new(&app_state);
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);

  let activity = domain::genre::stories::contribute::execute(
    &mut genre_repository,
    &mut activity_repository,
    domain::genre::stories::contribute::Input {
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

async fn create_genre(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::CreateGenreDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_genre_activity(
    app_state,
    session,
    domain::genre::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn update_genre(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::UpdateGenreDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let genre_id = path.into_inner();

  match create_genre_activity(
    app_state,
    session,
    domain::genre::stories::contribute::ChangeInput::Update {
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

async fn delete_genre(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let genre_id = path.into_inner();

  match create_genre_activity(
    app_state,
    session,
    domain::genre::stories::contribute::ChangeInput::Delete(genre_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("/contribute/genre", actix_web::web::post().to(create_genre))
    .route(
      "/contribute/genre/{genre_id}",
      actix_web::web::patch().to(update_genre),
    )
    .route(
      "/contribute/genre/{genre_id}",
      actix_web::web::delete().to(delete_genre),
    );
}
