use regex::Regex;
use rocket::Request;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome};
use std::ops::{Deref, DerefMut};

use crate::config::database::Db;
use crate::models::User;

pub struct CurrentUser(User);

impl Deref for CurrentUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CurrentUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let db = try_outcome!(request.guard::<Db>().await);
        let user = match token_str_from_request(request) {
            Some(token_str) => User::find_by_token_str(&db, token_str).await,
            None => None,
        };
        

        match user {
            Some(user) => Outcome::Success(CurrentUser(user)),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

fn token_str_from_request<'r>(request: &'r Request) -> Option<&'r str> {
    match request.headers().get_one("Authorization") {
        Some(auth_header) => {
            let regex = Regex::new(r#"Bearer (?P<token>\S+)"#).unwrap();
            match regex.captures(auth_header) {
                Some(captures) => {
                    match captures.name("token") {
                        Some(matching) => Some(matching.as_str()),
                        None => None,
                    }
                },
                None => None,
            }
        },
        None => None,
    }
}

