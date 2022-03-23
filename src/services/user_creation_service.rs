use crate::config::database::Db;
use crate::dtos::UserDto;
use crate::models::{users, User};
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use diesel::expression_methods::ExpressionMethods;
use std::fmt;

pub struct UserCreationService {
    pub db: Db,
}

#[derive(Debug)]
pub enum ApiExceptions {
    Conflict(String),
    InternalServerError(String)
}

impl fmt::Display for ApiExceptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl UserCreationService {
    pub fn new(db: Db) -> Self {
        UserCreationService { db }
    }

    pub async fn run(&self, user_dto: UserDto) -> Result<User, ApiExceptions> {
        self.db.run(move |conn| {
            let existing_user: Option<User> = users::table.filter(users::email.eq(&user_dto.email)).first(conn).ok();
            match existing_user {
                Some(_) => Err(ApiExceptions::Conflict("The email is already taken".to_string())),
                None => {
                    match diesel::insert_into(users::table).values(&user_dto).execute(conn) {
                        Ok(_) => {
                            users::table.filter(users::email.eq(&user_dto.email)).first::<User>(conn)
                                .map_err(|e| ApiExceptions::InternalServerError(e.to_string()))
                        },
                        Err(e) => Err(ApiExceptions::InternalServerError(e.to_string())),
                    }
                },
            }
        }).await
    }
}
