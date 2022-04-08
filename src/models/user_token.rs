use chrono::NaiveDateTime;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use rocket::http::Status;
use rocket::serde::Serialize;

use crate::config::database::Db;
use crate::dtos::UserTokenDto;
use crate::helpers::ErrorResponse;
use crate::models::User;
use crate::schema::{user_tokens, users};

#[derive(Debug, Serialize, Insertable, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct UserToken {
    #[serde(skip_serializing)]
    pub id: i32,
    #[serde(skip_serializing)]
    pub user_id: i32,
    pub token: String,
    #[serde(skip_serializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub updated_at: Option<NaiveDateTime>,
}

impl UserToken {
    pub async fn find_by_user_id(db: &Db, user_id: i32) -> Option<Self> {
        db.run(move |conn| 
            user_tokens::table.filter(user_tokens::user_id.eq(&user_id)).first(conn).ok()
        ).await
    }

    pub async fn find_by_token(db: &Db, token: String) -> Option<Self> {
        db.run(move |conn|
            user_tokens::table.filter(user_tokens::token.eq(&token)).first(conn).ok()
        ).await
    }

    pub async fn create(db: &Db, user_id: i32) -> Result<Self, ErrorResponse> {
        let user_token_dto = UserTokenDto::new(user_id);
        db.run(move |conn|
               match diesel::insert_into(user_tokens::table).values(&user_token_dto).execute(conn) {
                   Ok(_) => {
                       user_tokens::table.filter(user_tokens::user_id.eq(&user_id)).first::<UserToken>(conn)
                           .map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))
                   },
                   Err(e) => Err(ErrorResponse::new(Status::InternalServerError, e.to_string())),
               }
        ).await
    }

    pub async fn find_or_create(db: &Db, user_id: i32) -> Result<Self, ErrorResponse> {
        match Self::find_by_user_id(db, user_id).await {
            Some(token) => Ok(token),
            None => Ok(Self::create(db, user_id).await?),
        }
    }

    pub async fn user(&self, db: &Db) -> Option<User> {
        let user_id = self.user_id;

        db.run(move |conn|
            users::table.filter(users::id.eq(&user_id)).first(conn).ok()
        ).await
    }

    pub async fn destroy(&self, db: &Db) -> Result<usize, ErrorResponse> {
        let id = self.id;
        let size = db.run(move |conn|
            diesel::delete(user_tokens::table.filter(user_tokens::id.eq(&id))).execute(conn)
        ).await.map_err(|e| ErrorResponse::new(Status::InternalServerError, e.to_string()))?;

        Ok(size)
    }
}

