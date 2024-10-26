use actix_web::{
  dev::HttpServiceFactory,
  post,
  web::{self, Data, Json},
  HttpResponse, Responder,
};
use serde_json::json;

use crate::{
  domain::{
    self,
    user::{dto::UserRegisterDto, stories::CreateUserStoryError},
  },
  infra::sqlx::SqlUserRepository,
  shared::password_hash,
  AppState,
};

#[post("")]
async fn register_user(state: Data<AppState>, Json(data): Json<UserRegisterDto>) -> impl Responder {
  let mut user_repository = SqlUserRepository::new(state.db.clone());
  let password_hash = crate::infra::bcrypt::BcryptPasswordHash;
  let result = domain::user::stories::create_user(&mut user_repository, &password_hash, data).await;

  match result {
    Ok(data) => actix_web::HttpResponse::Ok().json(json!({
      "id": data.id,
      "username": Into::<String>::into(data.username)
    })),
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

pub fn controller() -> impl HttpServiceFactory {
  web::scope("/users").service(register_user)
}
