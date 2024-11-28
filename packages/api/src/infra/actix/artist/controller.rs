use actix_web::web;
use infra::actix::errors::ErrorResponse;

use crate::*;

use super::dto::ArtistPresenter;

async fn create_artist_activity(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  changes: domain::artist::stories::contribute::ChangeInput,
) -> Result<actix_web::HttpResponse, infra::actix::errors::ErrorResponse> {
  let actor_id = infra::actix::utils::get_actor_id(&session)?;

  let mut artist_repository = di::artist::repositories::ArtistRepository::new(&app_state);
  let mut activity_repository = di::activity::repositories::ActivityRepository::new(&app_state);

  let activity = domain::artist::stories::contribute::execute(
    &mut artist_repository,
    &mut activity_repository,
    domain::artist::stories::contribute::Input {
      actor_id,
      data: changes,
    },
  )
  .await
  .map_err(Into::<infra::actix::errors::ErrorResponse>::into)?;

  Ok(actix_web::HttpResponse::Created().json(Into::<
    infra::actix::activity::presenters::PublicActivityPresenter,
  >::into(activity)))
}

async fn create_artist(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::CreateArtistDTO>,
  session: actix_session::Session,
) -> impl actix_web::Responder {
  match create_artist_activity(
    app_state,
    session,
    domain::artist::stories::contribute::ChangeInput::Create(data.into()),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn update_artist(
  app_state: actix_web::web::Data<AppState>,
  actix_web::web::Json(data): actix_web::web::Json<super::dto::UpdateArtistDto>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_artist_activity(
    app_state,
    session,
    domain::artist::stories::contribute::ChangeInput::Update {
      id: artist_id,
      changes: (data.into()),
    },
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn delete_artist(
  app_state: actix_web::web::Data<AppState>,
  session: actix_session::Session,
  path: actix_web::web::Path<shared::vo::UUID4>,
) -> impl actix_web::Responder {
  let artist_id = path.into_inner();

  match create_artist_activity(
    app_state,
    session,
    domain::artist::stories::contribute::ChangeInput::Delete(artist_id),
  )
  .await
  {
    Ok(response) => response,
    Err(error) => error.into(),
  }
}

async fn find_by_slug(
  app_state: actix_web::web::Data<AppState>,
  path: actix_web::web::Path<shared::vo::Slug>,
) -> impl actix_web::Responder {
  let slug = path.into_inner();
  let mut artist_repository = di::artist::repositories::ArtistRepository::new(&app_state);

  match domain::artist::stories::find_by_slug::execute(&mut artist_repository, &slug).await {
    Ok(artist) => actix_web::HttpResponse::Ok().json(ArtistPresenter::from(artist)),
    Err(error) => ErrorResponse::from(error).into(),
  }
}

async fn find_all(
  app_state: actix_web::web::Data<AppState>,
  web::Query(page): web::Query<infra::actix::shared::PageParams>,
  web::Query(filter): web::Query<super::dto::FindAllQuery>,
) -> impl actix_web::Responder {
  let mut artist_repository = di::artist::repositories::ArtistRepository::new(&app_state);
  match domain::artist::stories::find_all::execute(
    &mut artist_repository,
    domain::artist::stories::find_all::Input {
      page: page.into(),
      search: filter.search,
      country: filter.country,
    },
  )
  .await
  {
    Ok(result) => {
      actix_web::HttpResponse::Ok().json(shared::paged::Paged::<ArtistPresenter>::from(result))
    }
    Err(err) => infra::actix::errors::ErrorResponse::from(err).into(),
  }
}

pub fn configure(config: &mut actix_web::web::ServiceConfig) {
  config
    .route("/artists", actix_web::web::get().to(find_all))
    .route("/artists/{slug}", actix_web::web::get().to(find_by_slug))
    .route(
      "/contribute/artist",
      actix_web::web::post().to(create_artist),
    )
    .route(
      "/contribute/artist/{artist_id}",
      actix_web::web::patch().to(update_artist),
    )
    .route(
      "/contribute/artist/{artist_id}",
      actix_web::web::delete().to(delete_artist),
    );
}
