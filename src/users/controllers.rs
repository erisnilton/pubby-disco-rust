use actix_web::{
    dev::HttpServiceFactory,
    post,
    web::{self, Json},
    Responder,
};

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users").service(create_user)
}

#[post("")]
async fn create_user(form: Json<super::dto::CreateUserDto>) -> impl Responder {
    println!("{:?}", form.email);
    return "OK";
}
