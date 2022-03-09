use crate::config::database::Db;
use crate::models::{User, users};
use rocket::response::{self, status::Created};
use rocket::serde::json::Json;
use diesel::query_dsl::RunQueryDsl;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, user: Json<User>) -> Result<Created<Json<User>>, response::Debug<diesel::result::Error>> {
    let user_value = user.clone();
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&user_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(user))
} 

