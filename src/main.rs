#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;

use rocket_sync_db_pools::{diesel, database};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage())
                   .mount("/", routes![index])
}

#[database("read_together_development")]
struct Db(diesel::MysqlConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("db/migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel MySQL Stage", |rocket| async {
        rocket.attach(Db::fairing())
              .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
