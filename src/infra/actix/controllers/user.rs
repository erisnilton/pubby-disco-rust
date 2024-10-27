use actix_session::Session;
use actix_web::{
  dev::HttpServiceFactory,
  get, post,
  web::{Data, Json},
  Responder,
};
use serde_json::json;

use crate::{
  di,
  domain::user::stories::{user_login, user_register},
  infra::actix::presenters::user::PublicUserPresenter,
  AppState,
};

#[post("")]
async fn register_user(
  state: Data<AppState>,
  Json(input): Json<user_register::Input>,
  session: Session,
) -> impl Responder {
  let mut user_repository = di::user::repositories::UserRepository::new(&state);
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;

  let register_result =
    user_register::execute(&mut user_repository, &password_hash, input.clone()).await;

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
        actix_web::HttpResponse::Created().json(Into::<PublicUserPresenter>::into(user))
      }
      Err(error) => error.into(),
    },
    Err(error) => error.into(),
  }
}

#[post("/login")]
async fn login(
  state: Data<AppState>,
  Json(data): Json<user_login::Input>,
  session: Session,
) -> impl Responder {
  let mut user_repository = di::user::repositories::UserRepository::new(&state);
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;
  let result = user_login::execute(&mut user_repository, &password_hash, data).await;

  match result {
    Ok(user) => {
      session.insert("disco_session", user.id.to_string()).ok();
      actix_web::HttpResponse::Ok().json(Into::<PublicUserPresenter>::into(user))
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

#[get("/me")]
async fn me(session: Session) -> impl Responder {
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

pub fn controller() -> impl HttpServiceFactory {
  actix_web::web::scope("/users")
    .service(register_user)
    .service(login)
    .service(me)
}
