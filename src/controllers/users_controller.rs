use crate::config::AppConfig;
use crate::config::database::Db;
use crate::guards::CurrentUser;
use crate::helpers::{ApiResponse, ErrorResponse};
use crate::dtos::{UserCreationDto, UserSignInDto};
use crate::services::{UserCreationService, UserTokenCreationService, UserSignInService, UserSignOutService};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, app_config: &State<AppConfig>, user: Json<UserCreationDto>) -> Result<ApiResponse<serde_json::Value, Option<serde_json::Value>>, ErrorResponse>
{
    let user = UserCreationService::new(&db).run(user.into_inner(), app_config).await?;
    let user_token = UserTokenCreationService::new(&db).run(user.id).await?;
    let value: serde_json::Value = json!({
        "user": user,
        "user_token": user_token,
    });

    Ok(ApiResponse::new(Status::Created, Some(value), None))
} 

#[post("/sign_in", data = "<user>")]
pub async fn sign_in(db: Db, user: Json<UserSignInDto>) -> Result<ApiResponse<serde_json::Value, Option<serde_json::Value>>, ErrorResponse> {
    let (user, token) = UserSignInService::new(&db).run(user.into_inner()).await?;
    let value: serde_json::Value = json!({
        "user": user,
        "user_token": token,
    });

    Ok(ApiResponse::new(Status::Ok, Some(value), None))
}

#[post("/sign_out")]
pub async fn sign_out(db: Db, current_user: CurrentUser) -> Result<ApiResponse<serde_json::Value, serde_json::Value>, ErrorResponse> {
    UserSignOutService::new(&db).run(current_user.into_inner()).await?;

    Ok(ApiResponse::new(Status::Ok, None, None))
}

