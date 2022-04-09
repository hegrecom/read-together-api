use rocket::serde::Deserialize;

use crate::schema::groups;

#[derive(Debug, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "groups"]
pub struct GroupCreationDto {
    pub name: String,
}

