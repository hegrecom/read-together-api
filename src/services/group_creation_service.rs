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
        Ok(Group::create(self.db, user, group_dto).await?)
    }
}

