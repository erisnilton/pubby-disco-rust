use actix_web::{body::BoxBody, web::JsonBody, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserPresenterDTO {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Responder for UserPresenterDTO {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        return actix_web::HttpResponse::Ok().json(self);
    }
}
