use crate::config::database::Db;
use crate::helpers::{ApiResponse, ErrorMessage};
use crate::models::User;
use crate::dtos::UserDto;
use crate::services::UserCreationService;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, user: Json<UserDto>) -> Result<ApiResponse<User, Option<String>>, ErrorMessage>
{
    let service = UserCreationService::new(db);
    let user = service.run(user.into_inner()).await?;

    Ok(ApiResponse::new(Status::Created, Some(user), None))
} 

