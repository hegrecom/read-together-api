use chrono::NaiveDateTime;
use diesel::Connection;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use rocket::serde::Serialize;

use crate::config::database::Db;
use crate::dtos::{GroupCreationDto, GroupMemberDto};
use crate::helpers::{ErrorFormatter, ErrorResponse};
use crate::models::User;
use crate::schema::{groups, group_members};

#[derive(Debug, Serialize, Insertable, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Group {
    pub async fn create(db: &Db, user: &User, group_dto: GroupCreationDto) -> Result<Self, ErrorResponse> {
        let user_id = user.id;
        db.run(move |conn| {
            conn.transaction(||
                diesel::insert_into(groups::table).values(&group_dto).execute(conn)
                    .and_then(|_|
                        groups::table.order(groups::id.desc()).first::<Self>(conn)
                            .and_then(|group| {
                                let member = GroupMemberDto::new(group.id, user_id, true);
                                let _ = diesel::insert_into(group_members::table).values(&member).execute(conn)?;
                                Ok(group)
                            })
                    )
            ).map_err(|e| ErrorFormatter::internal_server_error(e))
        }).await
    } 
}

