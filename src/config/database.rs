extern crate diesel_migrations;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::{diesel, database};

#[database("read_together")]
struct Db(diesel::MysqlConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    diesel_migrations::embed_migrations!("db/migrations");

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
