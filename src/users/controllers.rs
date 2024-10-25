use actix_web::{dev::HttpServiceFactory, get, post, web, Responder};
// use serde_json::json;

// use crate::{users::dto::PageParams, AppState};

// use crate::users::repository::UserRepositoryError::*;

pub fn controller() -> impl HttpServiceFactory {
  web::scope("/users").service(get_users).service(create_user)
}

#[get("")]
async fn get_users(//   state: web::Data<AppState>,
//   page_params: web::Query<PageParams>,
) -> impl Responder {
  // let user_repository = crate::infra::SqlXUserRepository::new(state.db.clone());

  // let page_params = page_params.into_inner();

  // match crate::users::stories::find_all(&user_repository, page_params).await {
  //     Ok(result) => actix_web::HttpResponse::Ok().json(result),
  //     Err(e) => match e {
  //         Conflict(message) => {
  //             actix_web::HttpResponse::Conflict().json(json!({ "message": message }))
  //         }
  //         InternalServerError(message) => {
  //             actix_web::HttpResponse::InternalServerError().json(json!({ "message": message }))
  //         }
  //     },
  // }
  actix_web::HttpResponse::Ok()
}

#[post("")]
async fn create_user(//   state: web::Data<AppState>,
//   form: Json<super::dto::CreateUserDto>,
) -> impl Responder {
  //   let user_repository = crate::infra::SqlXUserRepository::new(state.db.clone());

  //   return crate::users::stories::create(&user_repository, form.into_inner()).await;
  actix_web::HttpResponse::Ok()
}
