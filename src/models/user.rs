use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
    }
}

