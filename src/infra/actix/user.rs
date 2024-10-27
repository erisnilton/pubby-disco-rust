use actix_session::Session;
use actix_web::{
  cookie::{Cookie, CookieBuilder},
  dev::HttpServiceFactory,
  get, post,
  web::{scope, Data, Json},
  HttpResponse, Responder,
};
use serde_json::json;

use crate::{
  domain::{
    self,
    user::{
      dto::{UserLoginDto, UserRegisterDto},
      stories::{CreateUserStoryError, LoginError},
    },
  },
  infra::sqlx::SqlUserRepository,
  shared::password_hash,
  AppState,
};

#[post("")]
async fn register_user(
  state: Data<AppState>,
  Json(input): Json<UserRegisterDto>,
  session: Session,
) -> impl Responder {
  let mut user_repository = SqlUserRepository::new(state.db.clone());
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;
  let result =
    domain::user::stories::create_user(&mut user_repository, &password_hash, input.clone()).await;

  match result {
    Ok(user) => match domain::user::stories::login(
      &mut user_repository,
      &password_hash,
      UserLoginDto {
        username: input.username,
        password: input.password,
      },
    )
    .await
    {
      Err(_) => actix_web::HttpResponse::InternalServerError().json(json!({
        "name": "InternalServerError",
        "message": "Failed to login",
      })),
      Ok(user) => {
        session.insert("disco_session", user.id.to_string());
        actix_web::HttpResponse::Created().json(json!({
          "id": user.id,
          "username": Into::<String>::into(user.username)
        }))
      }
    },
    Err(CreateUserStoryError::InvalidInput(error)) => {
      actix_web::HttpResponse::BadRequest().json(json!({
        "name": "BadRequest",
        "message": "Invalid input",
        "details": error
      }))
    }
    Err(CreateUserStoryError::UserAlreadyExists) => {
      actix_web::HttpResponse::Conflict().json(json!({
        "name": "Conflict",
        "message": "User already exists",
      }))
    }
    Err(CreateUserStoryError::RepositoryError(error)) => {
      log::error!("Failed to create user: {:?}", error);

      actix_web::HttpResponse::InternalServerError().json(json!({
        "name": "InternalServerError",
        "message": "Failed to create user",
      }))
    }
  }
}

#[post("/login")]
async fn login(
  state: Data<AppState>,
  Json(data): Json<UserLoginDto>,
  session: Session,
) -> impl Responder {
  let mut user_repository = SqlUserRepository::new(state.db.clone());
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;
  let result = domain::user::stories::login(&mut user_repository, &password_hash, data).await;

  match result {
    Ok(user) => {
      session.insert("disco_session", user.id.to_string()).ok();
      actix_web::HttpResponse::Ok().json(json!({
        "id": user.id,
        "username": Into::<String>::into(user.username)
      }))
    }
    Err(domain::user::stories::LoginError::InvalidCredentials) => {
      actix_web::HttpResponse::Unauthorized().json(json!({
        "name": "Unauthorized",
        "message": "Invalid credentials",
      }))
    }
    Err(domain::user::stories::LoginError::RepositoryError(error)) => {
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
  scope("/users")
    .service(register_user)
    .service(login)
    .service(me)
}
