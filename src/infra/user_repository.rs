use std::borrow::Borrow;

use sqlx::{Pool, Postgres};

use crate::users::{
    dto::UserPresenterDTO,
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
    ) -> Result<
        Vec<crate::users::dto::UserPresenterDTO>,
        crate::users::repository::UserRepositoryError,
    > {
        let result = match sqlx::query!(
            r#"
            SELECT * FROM users
        "#
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("🔥 Failed to fetch users: {}", e);
                return Err(UserRepositoryError::InternalServerError(
                    "Failed to fetch users from database".to_string(),
                ));
            }
        };

        return Ok(result
            .into_iter()
            .map(|user| crate::users::dto::UserPresenterDTO {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
            })
            .collect::<Vec<_>>());
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
