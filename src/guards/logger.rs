use regex::Regex;
use rocket::{Request, Data, Response};
use rocket::request::{FromRequest, self};
use tracing::{Level, info, event};
use uuid::Uuid;

use crate::helpers::ErrorResponse;

pub struct Logger {
    request_id: String,
}

impl Logger {
    pub fn new() -> Self {
       Logger { request_id: Uuid::new_v4().to_string() } 
    }

    pub async fn log_request(&self, request: &Request<'_>, data: &mut Data<'_>) {
        let user_agent = request.headers().get_one("User-Agent").unwrap_or("");
        let user_ip = request.client_ip().and_then(|ip| Some(ip.to_string())).unwrap_or("".to_string());
        data.peek(512).await;
        let mut request_body = if data.peek_complete() {
            String::from_utf8_lossy(data.peek(512).await).to_string()
        } else { String::new() };
        request_body = self.filter_sensitive_info(request_body);
        info!(request_id = %self.request_id, uri = %request.uri(), method = %request.method(),
              user_agent = %user_agent, user_ip = %user_ip, %request_body);
    }

    fn filter_sensitive_info(&self, string: String) -> String {
        if string.is_empty() { return string }

        let filter_keys = vec!["password"];
        let mut filtered_string = string;
        for key in &filter_keys {
            let re = Regex::new(format!(r#""(?P<key>{})"[[:space:]]*:[[:space:]]*"(?P<value>[[:^space:]]+)""#, key).as_str()).unwrap();
            filtered_string = re.replace_all(&filtered_string, r#""$key" : "******""#).to_string();
        }

        filtered_string
    }

    pub async fn log_response<'r>(&self, response: &mut Response<'r>) {
        let status = response.status();
        let body = response.body_mut().to_string().await.unwrap_or("".to_string());
        info!(request_id = %self.request_id, %status, %body);
    }

    pub fn log(&self, level: Level, content: &str) {
        match level {
            Level::ERROR => event!(Level::ERROR, request_id = %self.request_id, content),
            Level::WARN => event!(Level::WARN, request_id = %self.request_id, content),
            Level::INFO => event!(Level::INFO, request_id = %self.request_id, content),
            Level::DEBUG => event!(Level::DEBUG, request_id = %self.request_id, content),
            Level::TRACE => event!(Level::TRACE, request_id = %self.request_id, content),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Logger {
    type Error = ErrorResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(request.local_cache(|| Logger::new()))
    }
}
