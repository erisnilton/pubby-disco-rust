use actix_web::{
    dev::HttpServiceFactory,
    get, post,
    web::{self, Json},
    Responder,
};
use serde_json::json;

use crate::AppState;

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users").service(get_users).service(create_user)
}

#[get("")]
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let result = match sqlx::query!(
        r#"
            SELECT * FROM users
        "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(result) => result,
        Err(e) => {
            eprintln!("🔥 Failed to fetch users: {}", e);
            return actix_web::HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "details": "Failed to fetch users from database"
            }));
        }
    };

    actix_web::HttpResponse::Ok().json(
        result
            .into_iter()
            .map(|user| crate::users::dto::UserPresenterDTO {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
            })
            .collect::<Vec<_>>(),
    )
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
            return actix_web::HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "details": "Failed to create user in database"
            }));
        }
    };

    actix_web::HttpResponse::Created().json(crate::users::dto::UserPresenterDTO {
        id: result.id.to_string(),
        name: result.name,
        email: result.email,
    })
}
