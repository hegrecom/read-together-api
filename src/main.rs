#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

mod config;

use config::database;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(database::stage())
                   .mount("/", routes![index])
}

