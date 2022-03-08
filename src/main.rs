#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod config;
mod models;

use config::database::{self, Db};
use models::{User, users};
use rocket::response::{self, status::Created};
use rocket::serde::json::Json;
use diesel::query_dsl::RunQueryDsl;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(database::stage())
                   .mount("/", routes![index])
                   .mount("/users", routes![sign_up])
}

#[post("/sign_up", data = "<user>")]
async fn sign_up(db: Db, user: Json<User>) -> Result<Created<Json<User>>, response::Debug<diesel::result::Error>> {
    let user_value = user.clone();
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&user_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(user))
} 


