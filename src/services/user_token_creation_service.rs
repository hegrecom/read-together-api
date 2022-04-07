use crate::config::database::Db;
use crate::helpers::ErrorResponse;
use crate::models::UserToken;

pub struct UserTokenCreationService<'a> {
    db: &'a Db
}

impl<'a> UserTokenCreationService<'a> {
    pub fn new(db: &'a Db) -> Self {
        UserTokenCreationService { db }
    }

    pub async fn run(&self, user_id: i32) -> Result<UserToken, ErrorResponse> {
        UserToken::create(self.db, user_id).await
    }
}

