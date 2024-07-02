use actix_web::{
    dev::HttpServiceFactory,
    post,
    web::{self, Json},
    Responder,
};

use crate::AppState;

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users").service(create_user)
}

#[post("")]
async fn create_user(
    state: web::Data<AppState>,
    form: Json<super::dto::CreateUserDto>,
) -> impl Responder {
    let result = match sqlx::query!(
        r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        form.name,
        form.email,
        form.password
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("🔥 Failed to create user: {}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    };

    actix_web::HttpResponse::Created().json(crate::users::dto::UserPresenterDTO {
        id: result.id.to_string(),
        name: result.name,
        email: result.email,
    })
}
