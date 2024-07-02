use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
    pub password: String,
}
