use crate::config::AppConfig;
use crate::config::database::Db;
use crate::helpers::{ApiResponse, ErrorResponse};
use crate::models::User;
use crate::dtos::UserDto;
use crate::services::UserCreationService;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, app_config: &State<AppConfig>, user: Json<UserDto>) -> Result<ApiResponse<User, Option<serde_json::Value>>, ErrorResponse>
{
    let service = UserCreationService::new(db);
    let user = service.run(user.into_inner(), app_config).await?;

    Ok(ApiResponse::new(Status::Created, Some(user), None))
} 

