use std::default;

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

#[derive(Debug, Serialize)]
pub struct Paged<T: Serialize> {
    pub items: Vec<T>,
    pub total_pages: u64,
    pub total_items: u64,
    pub current_page: u64,
    pub size: u64,
}

impl<T: Serialize> Paged<T> {
    pub fn new(
        items: Vec<T>,
        total_pages: u64,
        total_items: u64,
        current_page: u64,
        size: u64,
    ) -> Self {
        Paged {
            items,
            total_pages,
            total_items,
            current_page,
            size,
        }
    }

    pub fn from_total_items(items: Vec<T>, total_items: u64, params: &PageParams) -> Self {
        let total_pages = (total_items as f64 / params.size as f64).ceil() as u64;

        Paged {
            items,
            total_pages,
            total_items,
            current_page: params.page,
            size: params.size,
        }
    }
}

impl Responder for UserPresenterDTO {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        return actix_web::HttpResponse::Ok().json(self);
    }
}

impl<T: Serialize> Responder for Paged<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        return actix_web::HttpResponse::Ok().json(self);
    }
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_size")]
    pub size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_size() -> u64 {
    20
}

impl Default for PageParams {
    fn default() -> Self {
        PageParams { page: 1, size: 20 }
    }
}

impl PageParams {
    pub fn get_offset(&self) -> u64 {
        (self.page - 1) * self.size
    }
}
