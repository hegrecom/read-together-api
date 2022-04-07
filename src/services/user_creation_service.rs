use crate::config::AppConfig;
use crate::config::database::Db;
use crate::dtos::UserCreationDto;
use crate::helpers::ErrorResponse;
use crate::models::User;
use rocket::State;
use rocket::http::Status;

pub struct UserCreationService<'a> {
    db: &'a Db,
}

impl<'a> UserCreationService<'a> {
    pub fn new(db: &'a Db) -> Self {
        UserCreationService { db }
    }

    pub async fn run(&self, user_dto: UserCreationDto, app_config: &State<AppConfig>) -> Result<User, ErrorResponse> {
        let existing_user: Option<User> = User::find_by_email(&self.db, user_dto.email.clone()).await;
        match existing_user {
            Some(_) => Err(ErrorResponse::new(Status::Conflict, "The email has already taken".to_string())),
            None => User::create(&self.db, user_dto, app_config).await,
        }
    }
}

