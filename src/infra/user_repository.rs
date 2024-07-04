use actix_web::{http::StatusCode, HttpResponse, ResponseError, Result};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::users::{
    dto::{PageParams, Paged, UserPresenterDTO},
    repository::{UserRepository, UserRepositoryError},
};

pub struct SqlXUserRepository {
    pool: Pool<Postgres>,
}

impl SqlXUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl UserRepository for SqlXUserRepository {
    async fn find_all(
        &self,
        page_params: PageParams,
    ) -> Result<
        Paged<crate::users::dto::UserPresenterDTO>,
        crate::users::repository::UserRepositoryError,
    > {
        let count = match sqlx::query!("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
        {
            Ok(result) => result.count.unwrap_or(0),
            Err(e) => {
                eprintln!("🔥 Failed to fetch users count: {}", e);
                return Err(UserRepositoryError::InternalServerError(
                    "Failed to fetch users count from database".to_string(),
                ));
            }
        };

        match sqlx::query!(
            r#"
            SELECT * FROM users OFFSET $1 LIMIT $2
        "#,
            page_params.get_offset() as i64,
            page_params.size as i64
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(result) => {
                let items = result
                    .into_iter()
                    .map(|user| crate::users::dto::UserPresenterDTO {
                        id: user.id.to_string(),
                        name: user.name,
                        email: user.email,
                    })
                    .collect::<Vec<_>>();

                Ok(Paged::from_total_items(items, count as u64, &page_params))
            }
            Err(e) => {
                eprintln!("🔥 Failed to fetch users: {}", e);
                Err(UserRepositoryError::InternalServerError(
                    "Failed to fetch users from database".to_string(),
                ))
            }
        }
    }

    async fn create(
        &self,
        user: crate::users::dto::CreateUserDto,
    ) -> Result<crate::users::dto::UserPresenterDTO, crate::users::repository::UserRepositoryError>
    {
        return match sqlx::query!(
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
            user.name,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(result) => Ok(UserPresenterDTO {
                id: result.id.to_string(),
                name: result.name,
                email: result.email,
            }),
            Err(e) => match e {
                sqlx::Error::Database(db_error) => {
                    return match db_error.code() {
                        Some(code) if code == "23505" => Err(UserRepositoryError::Conflict(
                            "User with this email already exists".to_string(),
                        )),
                        _ => {
                            return Err(UserRepositoryError::InternalServerError(
                                "Failed to create user in database".to_string(),
                            ));
                        }
                    }
                }
                _ => {
                    eprintln!("🔥 Failed to create user: {}", e);
                    return Err(UserRepositoryError::InternalServerError(
                        "Failed to create user in database".to_string(),
                    ));
                }
            },
        };
    }
}

impl ResponseError for UserRepositoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserRepositoryError::Conflict(message) => HttpResponse::Conflict().json(json!({
                "error": "Conflict",
                "message": message
            })),
            UserRepositoryError::InternalServerError(message) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "details": message
                }))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserRepositoryError::Conflict(_) => StatusCode::CONFLICT,
            UserRepositoryError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
