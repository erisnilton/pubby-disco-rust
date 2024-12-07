use actix_session::Session;
use application::user::repository::UserRepository;
use domain::user::entity::User;
use shared::vo::UUID4;

use super::errors::ErrorResponse;

pub fn get_actor_id(session: &Session) -> Result<UUID4, ErrorResponse> {
  let actor_id = session
    .get::<String>("user_id")
    .map_err(|error| ErrorResponse::InternalServerError(error.to_string()))?
    .and_then(|v| UUID4::new(v).ok());

  if let Some(id) = actor_id {
    Ok(id)
  } else {
    Err(ErrorResponse::Forbidden(String::from(
      "user not authenticated",
    )))
  }
}

pub async fn get_actor(
  user_repository: &mut impl UserRepository,
  session: &Session,
) -> Result<User, ErrorResponse> {
  let actor_id = get_actor_id(session)?;

  let user = user_repository
    .find_by_id(actor_id)
    .await
    .map_err(|error| {
      println!("error: {:?}", error);
      ErrorResponse::InternalServerError(String::from("Failed to get user"))
    })?;

  match user {
    Some(user) => Ok(user),
    None => Err(ErrorResponse::Forbidden(String::from(
      "user not authenticated",
    ))),
  }
}
