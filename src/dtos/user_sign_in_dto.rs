use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSignInDto {
    pub email: String,
    pub password: String,
}

