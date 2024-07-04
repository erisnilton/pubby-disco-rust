use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, Json},
    Responder,
};
use serde_json::json;

use crate::{users::repository::UserRepositoryError, AppState};

use crate::users::repository::{UserRepository, UserRepositoryError::*};

use super::dto::UserPresenterDTO;

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users").service(get_users).service(create_user)
}

#[get("")]
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let user_repository = crate::infra::SqlXUserRepository::new(state.db.clone());

    match crate::users::stories::find_all(&user_repository).await {
        Ok(result) => actix_web::HttpResponse::Ok().json(result),
        Err(e) => match e {
            Conflict(message) => {
                actix_web::HttpResponse::Conflict().json(json!({ "message": message }))
            }
            InternalServerError(message) => {
                actix_web::HttpResponse::InternalServerError().json(json!({ "message": message }))
            }
        },
    }
}

#[post("")]
async fn create_user(
    state: web::Data<AppState>,
    form: Json<super::dto::CreateUserDto>,
) -> impl Responder {
    let user_repository = crate::infra::SqlXUserRepository::new(state.db.clone());

    return crate::users::stories::create(&user_repository, form.into_inner()).await;
}
