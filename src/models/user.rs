use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    password: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

