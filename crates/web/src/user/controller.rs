use actix_session::Session;
use actix_web::{
  get, post,
  web::{Data, Json},
  HttpResponse, Responder,
};

use application::user;
use serde_json::json;

use crate::{app_state::AppState, di, errors::ErrorResponse, utils::get_actor};

use super::dto::UserRegisterInputDTO;

#[post("/register")]
async fn register_user(
  state: Data<AppState>,
  Json(input): Json<UserRegisterInputDTO>,
  session: Session,
) -> HttpResponse {
  let mut user_repository = di::user::user_repository(&state);
  let password_hash = di::ports::password_hash();

  let register_result = application::user::stories::user_register::execute(
    &mut user_repository,
    &password_hash,
    input.clone().into(),
  )
  .await;

  match register_result {
    Ok(_) => match application::user::stories::user_login::execute(
      &mut user_repository,
      &password_hash,
      application::user::stories::user_login::Input {
        username: input.username,
        password: input.password,
      },
    )
    .await
    {
      Ok(user) => {
        session.insert("user_id", user.id().to_string()).ok();
        actix_web::HttpResponse::Ok()
          .json(Into::<super::presenters::PublicUserPresenter>::into(user))
      }
      Err(error) => ErrorResponse::from(error).into(),
    },
    Err(error) => ErrorResponse::from(error).into(),
  }
}

#[post("/login")]
async fn user_login(
  state: Data<AppState>,
  Json(data): Json<super::dto::UserLoginInputDTO>,
  session: Session,
) -> impl Responder {
  let mut user_repository = di::user::user_repository(&state);
  let password_hash = di::ports::password_hash();
  let result = application::user::stories::user_login::execute(
    &mut user_repository,
    &password_hash,
    application::user::stories::user_login::Input {
      username: data.username,
      password: data.password,
    },
  )
  .await;

  match result {
    Ok(user) => {
      session.insert("user_id", user.id().to_string()).ok();
      actix_web::HttpResponse::Ok().json(Into::<super::presenters::PublicUserPresenter>::into(user))
    }
    Err(application::user::stories::user_login::LoginError::InvalidCredentials) => {
      actix_web::HttpResponse::Unauthorized().json(json!({
        "name": "Unauthorized",
        "message": "Invalid credentials",
      }))
    }
    Err(application::user::stories::user_login::LoginError::RepositoryError(error)) => {
      log::error!("Failed to login: {:?}", error);

      actix_web::HttpResponse::InternalServerError().json(json!({
        "name": "InternalServerError",
        "message": "Failed to login",
      }))
    }
  }
}

#[get("/users/me")]
async fn get_me(state: Data<AppState>, session: Session) -> impl Responder {
  let mut user_repository = di::user::user_repository(&state);
  let user = get_actor(&mut user_repository, &session)
    .await
    .map_err(ErrorResponse::from);

  match user {
    Ok(user) => {
      actix_web::HttpResponse::Ok().json(Into::<super::presenters::PublicUserPresenter>::into(user))
    }
    Err(error) => error.into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .service(register_user)
    .service(user_login)
    .service(get_me);
}
