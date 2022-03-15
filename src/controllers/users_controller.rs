use crate::config::database::Db;
use crate::models::users;
use crate::dtos::UserDto;
use rocket::response::{self, status::Created};
use rocket::serde::json::Json;
use diesel::query_dsl::RunQueryDsl;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, user: Json<UserDto>) -> Result<Created<Json<&'static str>>, response::Debug<diesel::result::Error>> {
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&*user)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(Json::from("{}")))
} 

