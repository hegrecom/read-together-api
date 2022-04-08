use crate::config::database::Db;
use crate::helpers::ErrorResponse;
use crate::models::{User, UserToken};

pub struct UserSignOutService<'a> {
    db: &'a Db
}

impl<'a> UserSignOutService<'a> {
    pub fn new(db: &'a Db) -> Self {
        UserSignOutService { db }
    }

    pub async fn run(&self, user: User) -> Result<usize, ErrorResponse> {
        if let Some(token) = UserToken::find_by_user_id(self.db, user.id).await {
            Ok(token.destroy(self.db).await?)
        } else {
            Ok(0)
        }
    }
}

