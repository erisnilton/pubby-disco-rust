use actix_session::Session;
use actix_web::{
  web::{post, Data, Json},
  HttpResponse, Responder,
};

use crate::{
  domain::{self, activity::stories::CreateActivityInput, user::User},
  infra::{
    actix::errors::ErrorResponse,
    sqlx::{SqlxActivityRepository, SqlxGenreRepository, SqlxUserRepository},
  },
  AppState,
};

async fn create_activity(
  state: Data<AppState>,
  Json(data): Json<super::dto::CreateActivityDto>,
  session: Session,
) -> impl Responder {
  let mut activity_repository = SqlxActivityRepository::new(state.db.clone());
  let mut genre_repository = SqlxGenreRepository::new(state.db.clone());
  let mut user_repository = SqlxUserRepository::new(&state);
  let actor = crate::infra::actix::utils::get_actor(&mut user_repository, &session).await;

  if let Err(error) = actor {
    return error.into();
  }

  let actor = actor.unwrap();

  let result = domain::activity::stories::create_activity(
    &mut activity_repository,
    &mut genre_repository,
    CreateActivityInput {
      data: data.into(),
      user: actor,
    },
  )
  .await;

  match result {
    Ok(data) => HttpResponse::Created().json(
      Into::<super::presenters::PublicActivityPresenter>::into(data),
    ),
    Err(error) => Into::<ErrorResponse>::into(error).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config.route("/activities", post().to(create_activity));
}
