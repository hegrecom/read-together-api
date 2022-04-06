use chrono::NaiveDateTime;
use diesel_derive_enum::DbEnum;
use rocket::serde::Serialize;

#[derive(DbEnum, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Activity {
    Start,
    Read,
}

#[derive(Debug, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct PlanContentUserActivity {
    pub id: i32,
    pub plan_content_id: i32,
    pub user_id: i32,
    pub activity: Activity,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

