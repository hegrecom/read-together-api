use rocket::http::Status;

use crate::config::database::Db;
use crate::dtos::UserSignInDto;
use crate::helpers::ErrorResponse;
use crate::models::{User, UserToken};

pub struct UserSignInService<'a> {
    db: &'a Db
}

impl<'a> UserSignInService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub async fn run(&self, user_dto: UserSignInDto) -> Result<(User, UserToken), ErrorResponse> {
        let user = User::find_by_email(self.db, user_dto.email.clone()).await;
        match user {
            Some(user) => {
                if user.verify_password(&user_dto)? {
                    let token = UserToken::find_or_create(self.db, user.id).await?; 
                    Ok((user, token))
                } else {
                    Err(self.credential_invalid_response())
                }
            },
            None => Err(self.credential_invalid_response()),
        }
    } 

    fn credential_invalid_response(&self) -> ErrorResponse {
        ErrorResponse::new(Status::Unauthorized, "Credential is invalid".to_string())
    }
}

