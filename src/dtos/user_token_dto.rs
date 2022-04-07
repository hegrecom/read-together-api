use rocket::serde::Deserialize;
use uuid::Uuid;

use crate::schema::user_tokens;

#[derive(Debug, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "user_tokens"]
pub struct UserTokenDto {
    #[serde(skip_deserializing)]
    pub user_id: i32,
    pub token: String,
}

impl UserTokenDto {
    pub fn new(user_id: i32) -> Self {
        UserTokenDto { user_id, token: Uuid::new_v4().to_string() }        
    }
}

