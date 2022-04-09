use rocket::http::Status;
use validator::{ValidationErrors, ValidationErrorsKind};
use std::borrow::Cow;

use crate::helpers::ErrorResponse;

pub struct ErrorFormatter;

impl ErrorFormatter {
    pub fn format_error(validation_errors: ValidationErrors) -> String {
        let mut message = String::new();
        for (key, value) in validation_errors.into_errors() {
            if !message.is_empty() {
                message.push_str("\n");
            }
            message.push_str(&format!("{}: ", key));
            match value {
                ValidationErrorsKind::Struct(errors) => message.push_str(&ErrorFormatter::format_error(*errors)),
                ValidationErrorsKind::List(errors) => {
                    for (_, value) in errors {
                        message.push_str(&ErrorFormatter::format_error(*value));
                    }
                },
                ValidationErrorsKind::Field(errors) => message.push_str(&errors.into_iter().map(|error| error.message.unwrap_or(Cow::from(""))).collect::<Vec<Cow<'static, str>>>().join(", ")),
            }
        }

        message
    }

    pub fn internal_server_error(error: impl ToString) -> ErrorResponse {
        ErrorResponse::new(Status::InternalServerError, error.to_string())
    }
}

