use chrono::NaiveDateTime;
use diesel::expression_methods::ExpressionMethods;
use rocket::http::Status;
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use rocket::serde::Serialize;
use rocket::State;
use validator::Validate;

use crate::config::AppConfig;
use crate::config::database::Db;
use crate::dtos::{UserCreationDto, UserSignInDto};
use crate::helpers::{ErrorResponse, ErrorFormatter};
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub async fn find_by_email(db: &Db, email: String) -> Option<Self> {
        db.run(move |conn|
            users::table.filter(users::email.eq(&email)).first(conn).ok()
        ).await
    }

    pub async fn create(db: &Db, mut user_dto: UserCreationDto, app_config: &State<AppConfig>) -> Result<Self, ErrorResponse> {
        user_dto.validate().map_err(|e| ErrorResponse::new(Status::BadRequest, ErrorFormatter::format_error(e)))?;
        user_dto.encrypt_password(app_config.get_password_salt()).map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))?;
        db.run(move |conn|
            match diesel::insert_into(users::table).values(&user_dto).execute(conn) {
                Ok(_) => {
                    users::table.filter(users::email.eq(&user_dto.email)).first::<User>(conn)
                        .map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))
                },
                Err(e) => Err(ErrorResponse::new(Status::InternalServerError, e.to_string())),
            }
        ).await
    }

    pub fn verify_password(&self, user_dto: &UserSignInDto) -> Result<bool, ErrorResponse> {
        let matched = bcrypt::verify(&user_dto.password, &self.password).map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))?;

        Ok(matched)
    }
}

