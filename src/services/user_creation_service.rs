use crate::config::AppConfig;
use crate::config::database::Db;
use crate::dtos::UserDto;
use crate::helpers::{ErrorResponse, ErrorFormatter};
use crate::models::{users, User};
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use diesel::expression_methods::ExpressionMethods;
use rocket::State;
use rocket::http::Status;
use validator::Validate;

pub struct UserCreationService {
    pub db: Db,
}

impl UserCreationService {
    pub fn new(db: Db) -> Self {
        UserCreationService { db }
    }

    pub async fn run(&self, mut user_dto:UserDto, app_config: &State<AppConfig>) -> Result<User, ErrorResponse> {
        user_dto.validate().map_err(|e| ErrorResponse::new(Status::BadRequest, ErrorFormatter::format_error(e)))?;
        user_dto.encrypt_password(app_config.get_password_salt()).map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))?;
        self.db.run(move |conn| {
            let existing_user: Option<User> = users::table.filter(users::email.eq(&user_dto.email)).first(conn).ok();
            match existing_user {
                Some(_) => Err(ErrorResponse::new(Status::Conflict, "The email is already taken".to_string())),
                None => {
                    match diesel::insert_into(users::table).values(&user_dto).execute(conn) {
                        Ok(_) => {
                            users::table.filter(users::email.eq(&user_dto.email)).first::<User>(conn)
                                .map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))
                        },
                        Err(e) => Err(ErrorResponse::new(Status::InternalServerError, e.to_string())),
                    }
                },
            }
        }).await
    }
}
