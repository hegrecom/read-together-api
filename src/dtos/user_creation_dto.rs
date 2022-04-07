use crate::schema::users;
use bcrypt::{self, BcryptError};
use regex::Regex;
use rocket::serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Insertable, Validate)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UserCreationDto {
    #[validate(email(code = "invalid_email_format", message = "Email format is invalid"))]
    pub email: String,
    #[validate(custom(function = "validate_password", message = "Password should include at least 1 lower case letter, 1 upper case letter, 1 digit and special character between 8 to 32"))]
    pub password: String,
    pub name: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let length = Regex::new(".{8,32}").unwrap();
    let lower_case = Regex::new("[a-z]+").unwrap();
    let upper_case = Regex::new("[A-Z]+").unwrap();
    let digit = Regex::new(r"\d+").unwrap();
    let special_chars = Regex::new(r"[\*!@\$%\^&\(\)\{\}\[\]:;<>,\.\?/~_\+\-=\|\\]+").unwrap();

    if length.is_match(password) && lower_case.is_match(password) 
        && upper_case.is_match(password) && digit.is_match(password)
        && special_chars.is_match(password) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}

impl UserCreationDto {
    pub fn encrypt_password(&mut self, salt: &[u8]) -> Result<(), BcryptError> {
        match bcrypt::hash_with_salt(self.password.to_owned(), 10, salt) {
            Ok(result) => { 
                self.password = result.to_string();
                Ok(())
            },
            Err(error) => { Err(error) },
        }
    }
}

