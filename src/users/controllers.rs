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
async fn create_user(
    pool: web::Data<sqlx::PgPool>,
    form: Json<super::dto::CreateUserDto>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        form.name,
        form.email,
        form.password
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    println!("User created with id: {}", result.id);

    return "OK";
}
