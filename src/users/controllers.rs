use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, Json},
    Responder,
};
use serde_json::json;

use crate::AppState;

use crate::users::repository::{UserRepository, UserRepositoryError::*};

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users").service(get_users).service(create_user)
}

#[get("")]
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let user_repository = crate::infra::SqlXUserRepository::new(state.db.clone());

    return match user_repository.find_all().await {
        Ok(result) => actix_web::HttpResponse::Ok().json(result),
        Err(_) => actix_web::HttpResponse::InternalServerError()
            .json(json!({ "error": "Internal Server Error" })),
    };
}

#[post("")]
async fn create_user(
    state: web::Data<AppState>,
    form: Json<super::dto::CreateUserDto>,
) -> impl Responder {
    let user_repository = crate::infra::SqlXUserRepository::new(state.db.clone());

    return match user_repository.create(form.into_inner()).await {
        Ok(result) => actix_web::HttpResponse::Ok().json(result),
        Err(e) => match e {
            Conflict(message) => {
                return actix_web::HttpResponse::Conflict().json(json!({
                    "error": "Conflict",
                    "message": message
                }));
            }
            InternalServerError(message) => {
                return actix_web::HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "details": message
                }));
            }
        },
    };
}
