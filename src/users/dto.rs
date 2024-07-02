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
