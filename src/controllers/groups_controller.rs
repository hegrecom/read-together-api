use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json;

use crate::config::database::Db;
use crate::dtos::GroupCreationDto;
use crate::guards::CurrentUser;
use crate::helpers::{ApiResponse, ErrorResponse};
use crate::models::Group;
use crate::services::GroupCreationService;

#[post("/", data = "<group>")]
pub async fn create(db: Db, current_user: CurrentUser, group: Json<GroupCreationDto>) -> Result<ApiResponse<Group, serde_json::Value>, ErrorResponse> {
    let group = GroupCreationService::new(&db).run(&current_user, group.into_inner()).await?;

    Ok(ApiResponse::new(Status::Created, Some(group), None))
}

