use actix_web::{dev::HttpServiceFactory, web};

mod controller;

pub fn controller() -> impl HttpServiceFactory {
  web::scope("/activity").service(controller::create_activity)
}
