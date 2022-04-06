#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use figment::providers::Serialized;
use rocket::figment::Figment;
use rocket::fairing::AdHoc;

mod config;
mod controllers;
mod dtos;
mod guards;
mod helpers;
mod models;
mod services;
mod schema;
mod fairings;

use config::{database, AppConfig};
use controllers::users_controller;
use fairings::LoggerFairing;

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::figment())
        .merge(Serialized::defaults(AppConfig::config()));
        
    rocket::custom(figment)
        .attach(AdHoc::config::<AppConfig>())
        .attach(database::stage())
        .attach(LoggerFairing)
        .mount("/users", routes![users_controller::sign_up])
        .register("/", catchers![helpers::default_catcher])
}

