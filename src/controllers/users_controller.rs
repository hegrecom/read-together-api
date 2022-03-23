use crate::config::database::Db;
use crate::models::User;
use crate::dtos::UserDto;
use crate::services::{UserCreationService, ApiExceptions};
use rocket::response::{self, status::Created};
use rocket::serde::json::Json;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, user: Json<UserDto>) -> Result<Created<Json<User>>, response::Debug<ApiExceptions>> {
    let service = UserCreationService::new(db);
    let user = service.run(user.into_inner()).await?;

    Ok(Created::new("/").body(Json::from(user)))
} 

