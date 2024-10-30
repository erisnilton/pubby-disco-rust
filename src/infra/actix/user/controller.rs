use actix_session::Session;
use actix_web::{
  web::{get, post, Data, Json},
  HttpResponse, Responder,
};

use serde_json::json;

use crate::{
  di,
  domain::user::stories::{user_login, user_register},
  infra::actix::errors::ErrorResponse,
  AppState,
};

pub async fn register_user(
  state: Data<AppState>,
  Json(input): Json<super::dto::UserRegisterInputDTO>,
  session: Session,
) -> HttpResponse {
  let mut user_repository = di::user::repositories::UserRepository::new(&state);
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;

  let register_result =
    user_register::execute(&mut user_repository, &password_hash, input.clone().into()).await;

  match register_result {
    Ok(_) => match user_login::execute(
      &mut user_repository,
      &password_hash,
      user_login::Input {
        username: input.username,
        password: input.password,
      },
    )
    .await
    {
      Ok(user) => {
        session.insert("user_id", user.id.to_string()).ok();
        actix_web::HttpResponse::Ok()
          .json(Into::<super::presenters::PublicUserPresenter>::into(user))
      }
      Err(error) => Into::<ErrorResponse>::into(error).into(),
    },
    Err(error) => Into::<ErrorResponse>::into(error).into(),
  }
}

pub async fn user_login(
  state: Data<AppState>,
  Json(data): Json<super::dto::UserLoginInputDTO>,
  session: Session,
) -> impl Responder {
  let mut user_repository = di::user::repositories::UserRepository::new(&state);
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;
  let result = user_login::execute(&mut user_repository, &password_hash, data.into()).await;

  match result {
    Ok(user) => {
      session.insert("disco_session", user.id.to_string()).ok();
      actix_web::HttpResponse::Ok().json(Into::<super::presenters::PublicUserPresenter>::into(user))
    }
    Err(user_login::LoginError::InvalidCredentials) => actix_web::HttpResponse::Unauthorized()
      .json(json!({
        "name": "Unauthorized",
        "message": "Invalid credentials",
      })),
    Err(user_login::LoginError::RepositoryError(error)) => {
      log::error!("Failed to login: {:?}", error);

      actix_web::HttpResponse::InternalServerError().json(json!({
        "name": "InternalServerError",
        "message": "Failed to login",
      }))
    }
  }
}

async fn get_me(session: Session) -> impl Responder {
  match session.get::<String>("disco_session") {
    Ok(Some(user_id)) => actix_web::HttpResponse::Ok().json(json!({
      "id": user_id,
    })),
    _ => actix_web::HttpResponse::Unauthorized().json(json!({
      "name": "Unauthorized",
      "message": "Unauthorized",
    })),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("login", post().to(user_login))
    .route("me", get().to(get_me))
    .route("register", post().to(register_user));
}
