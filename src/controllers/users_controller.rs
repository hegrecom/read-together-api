use crate::config::database::Db;
use crate::models::{users, User};
use crate::dtos::UserDto;
use rocket::response::{self, status::Created};
use rocket::serde::json::Json;
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use diesel::expression_methods::ExpressionMethods;

#[post("/sign_up", data = "<user>")]
pub async fn sign_up(db: Db, user: Json<UserDto>) -> Result<Created<Json<User>>, response::Debug<diesel::result::Error>> {
    let user: User = db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&*user)
            .execute(conn)?;
        users::table.filter(users::email.eq(&user.email)).first(conn)
    }).await?;

    Ok(Created::new("/").body(Json::from(user)))
} 

