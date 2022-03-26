use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Request, Response};
use rocket::serde::Serialize;
use serde_json;
use std::io::Cursor;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T, U> where T: Serialize, U: Serialize {
    #[serde(skip_serializing)]
    pub status: Status,
    data: Option<T>,
    meta: Option<U>,
}

impl<'r, T, U> Responder<'r, 'static> for ApiResponse<T, U> where T: Serialize, U: Serialize {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let response_body = serde_json::to_string(&self).map_err(|_| Status::InternalServerError)?;
        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(response_body.len(), Cursor::new(response_body))
            .ok()
    }
}

impl<T, U> ApiResponse<T, U> where T: Serialize, U: Serialize {
    pub fn new(status: Status, data: Option<T>, meta: Option<U>) -> Self {
        ApiResponse { status, data, meta }
    }
}

impl<T> ApiResponse<T, ErrorResponse> where T: Serialize {
    pub fn from(error_message: ErrorResponse) -> Self {
        ApiResponse::new(error_message.status, None, Some(error_message))
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub status: Status,
    message: String,
}

impl ErrorResponse {
    pub fn new(status: Status, message: String) -> Self {
        ErrorResponse { status, message }
    }
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        ApiResponse::<Option<serde_json::Value>, ErrorResponse>::from(self).respond_to(request)
    }
}

