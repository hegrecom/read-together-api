use rocket::serde::{de, Deserialize, Deserializer};
use crate::models::users;
use bcrypt;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UserDto {
    pub email: String,
    #[serde(deserialize_with = "encrypt_password")]
    pub password: String,
    pub name: String,
}

fn encrypt_password<'de, D>(deserializer: D) -> Result<String, D::Error>
where D: Deserializer<'de> 
{
    let password: &str = Deserialize::deserialize(deserializer)?;

    match bcrypt::hash_with_salt(password, 10, b"aaaaaaaaaaaaaaaa") {
        Ok(result) => Ok(result.to_string()),
        Err(error) => Err(error).map_err(de::Error::custom),
    }
}

