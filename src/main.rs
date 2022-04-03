#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod config;
mod controllers;
mod dtos;
mod guards;
mod helpers;
mod models;
mod services;
mod fairings;

use config::database;
use controllers::users_controller;
use fairings::LoggerFairing;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(database::stage())
                   .attach(LoggerFairing)
                   .mount("/users", routes![users_controller::sign_up])
                   .register("/", catchers![helpers::default_catcher])
}

