use actix_web::{
  dev::HttpServiceFactory,
  post,
  web::{self, Data, Json},
  Responder,
};
use serde_json::json;

use crate::{
  domain::{
    self,
    activity::{dto::CreateActivityDto, stories::CreateActivityInput, ActivityRepositoryError},
    user::User,
  },
  infra::sqlx::{SqlxActivityRepository, SqlxGenreRepository},
  shared::vo::UUID4,
  AppState,
};

#[post("")]
async fn create_activity(
  state: Data<AppState>,
  Json(data): Json<CreateActivityDto>,
) -> impl Responder {
  let mut activity_repository = SqlxActivityRepository::new(state.db.clone());
  let mut genre_repository = SqlxGenreRepository::new(state.db.clone());
  let user = User {
    id: UUID4::new("e4442384-61ea-440d-be63-cb2642e58007").unwrap_or_default(),
    ..Default::default()
  };
  let result = domain::activity::stories::create_activity(
    &mut activity_repository,
    &mut genre_repository,
    CreateActivityInput { data, user },
  )
  .await;

  match result {
    Ok(data) => actix_web::HttpResponse::Ok().json(json!({
      "id": data.id,
      "status": Into::<String>::into(data.status)
    })),
    Err(ActivityRepositoryError::EntityNotFound) => actix_web::HttpResponse::UnprocessableEntity()
      .json(json!({
        "name": "UnprocessableEntity",
        "message": "Failed to find entity"
      })),
    Err(ActivityRepositoryError::InternalServerError(err)) => {
      actix_web::HttpResponse::InternalServerError().json(json!({
        "name": "Internal Server Error",
        "message": err
      }))
    }
  }
}

pub fn controller() -> impl HttpServiceFactory {
  web::scope("/activities").service(create_activity)
}
