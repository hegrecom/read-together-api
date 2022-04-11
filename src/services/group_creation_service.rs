use rocket::http::Status;

use crate::config::database::Db;
use crate::dtos::GroupCreationDto;
use crate::helpers::ErrorResponse;
use crate::models::{Group, User};


pub struct GroupCreationService<'a> {
    db: &'a Db,
}

impl<'a> GroupCreationService<'a> {
    pub fn new(db: &'a Db) -> Self {
        GroupCreationService { db }
    }

    pub async fn run(&self, user: &User, group_dto: GroupCreationDto) -> Result<Group, ErrorResponse> {
        let existing_group_name: Option<Group> = Group::find_by_name(self.db, group_dto.name.clone()).await;
        match existing_group_name {
            Some(_) => Err(ErrorResponse::new(Status::Conflict, "The group name has already taken".to_string())),
            None => Ok(Group::create(self.db, user, group_dto).await?),
        }
    }
}

