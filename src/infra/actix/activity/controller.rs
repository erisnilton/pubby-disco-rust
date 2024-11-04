use actix_session::Session;
use actix_web::{
  web::{patch, post, Data, Json},
  HttpResponse, Responder,
};

use crate::{
  domain::{self, activity::stories::CreateActivityInput},
  infra::{
    actix::errors::ErrorResponse,
    sqlx::{SqlxActivityRepository, SqlxGenreRepository, SqlxUserRepository},
  },
  shared::vo::UUID4,
  AppState,
};
async fn create_activity(
  state: Data<AppState>,
  Json(data): Json<super::dto::CreateActivityDto>,
  session: Session,
) -> impl Responder {
  print!("{:?}", session.entries());
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

async fn aprove_activity(
  state: Data<AppState>,
  Json(data): Json<super::dto::ApproveActivityDto>,
  session: Session,
) -> impl Responder {
  let mut activity_repository = SqlxActivityRepository::new(state.db.clone());
  let mut user_repository = SqlxUserRepository::new(&state);
  let mut genre_repository = SqlxGenreRepository::new(state.db.clone());
  let actor = crate::infra::actix::utils::get_actor(&mut user_repository, &session).await;
  if let Err(error) = actor {
    return error.into();
  }
  let actor = actor.unwrap();
  let result = domain::activity::stories::approve::execute(
    &mut activity_repository,
    &mut genre_repository,
    domain::activity::stories::approve::Input {
      activity_id: UUID4::new(data.activity_id).unwrap_or_default(),
      actor,
    },
  )
  .await;

  match result {
    Ok(data) => HttpResponse::Ok().json(Into::<super::presenters::PublicActivityPresenter>::into(
      data,
    )),
    Err(error) => Into::<ErrorResponse>::into(error).into(),
  }
}

async fn reject_activity(
  state: Data<AppState>,
  Json(data): Json<super::dto::RejectActivityDto>,
  session: Session,
) -> impl Responder {
  let mut activity_repository = SqlxActivityRepository::new(state.db.clone());
  let mut user_repository = SqlxUserRepository::new(&state);
  let actor = crate::infra::actix::utils::get_actor(&mut user_repository, &session).await;
  if let Err(error) = actor {
    return error.into();
  }
  let actor = actor.unwrap();
  let result = domain::activity::stories::reject::execute(
    &mut activity_repository,
    domain::activity::stories::reject::Input {
      activity_id: UUID4::new(data.activity_id).unwrap_or_default(),
      reason: data.reason.clone(),
      user: actor,
    },
  )
  .await;

  match result {
    Ok(data) => HttpResponse::Ok().json(Into::<super::presenters::PublicActivityPresenter>::into(
      data,
    )),
    Err(error) => Into::<ErrorResponse>::into(error).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("/activities", post().to(create_activity))
    .route("/activities/reject", patch().to(reject_activity))
    .route("/activities/approve", patch().to(aprove_activity));
}
