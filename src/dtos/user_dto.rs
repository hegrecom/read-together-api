use rocket::serde::Deserialize;
use crate::models::users;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UserDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

