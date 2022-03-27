#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod config;
mod controllers;
mod dtos;
mod helpers;
mod models;
mod services;

use config::database;
use controllers::users_controller;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(database::stage())
                   .mount("/users", routes![users_controller::sign_up])
                   .register("/", catchers![helpers::default_catcher])
}

