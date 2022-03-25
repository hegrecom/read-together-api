use crate::config::database::Db;
use crate::dtos::UserDto;
use crate::helpers::ErrorMessage;
use crate::models::{users, User};
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use diesel::expression_methods::ExpressionMethods;
use rocket::http::Status;
use validator::Validate;

pub struct UserCreationService {
    pub db: Db,
}

impl UserCreationService {
    pub fn new(db: Db) -> Self {
        UserCreationService { db }
    }

    pub async fn run(&self, mut user_dto:UserDto) -> Result<User, ErrorMessage> {
        user_dto.validate().map_err(|e| ErrorMessage::new(Status::BadRequest, e.to_string()))?;
        user_dto.encrypt_password().map_err(|e| ErrorMessage::new(Status::InternalServerError, e.to_string()))?;
        self.db.run(move |conn| {
            let existing_user: Option<User> = users::table.filter(users::email.eq(&user_dto.email)).first(conn).ok();
            match existing_user {
                Some(_) => Err(ErrorMessage::new(Status::Conflict, "The email is already taken".to_string())),
                None => {
                    match diesel::insert_into(users::table).values(&user_dto).execute(conn) {
                        Ok(_) => {
                            users::table.filter(users::email.eq(&user_dto.email)).first::<User>(conn)
                                .map_err(|e| ErrorMessage::new(Status::InternalServerError, e.to_string()))
                        },
                        Err(e) => Err(ErrorMessage::new(Status::InternalServerError, e.to_string())),
                    }
                },
            }
        }).await
    }
}
