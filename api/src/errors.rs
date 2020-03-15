// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
pub struct Errors {
    errors: ValidationErrors,
}

pub type FieldName = &'static str;
pub type FieldErrorCode = &'static str;

impl Errors {
    pub fn new(errs: &[(FieldName, FieldErrorCode)]) -> Self {
        let mut errors = ValidationErrors::new();
        for (field, code) in errs {
            errors.add(field, ValidationError::new(code));
        }
        Self { errors }
    }
}

impl<'r> Responder<'r> for Errors {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        use validator::ValidationErrorsKind::Field;

        let mut errors = json!({});
        for (field, field_errors) in self.errors.into_errors() {
            if let Field(field_errors) = field_errors {
                errors[field] = field_errors.into_iter().map(|field_error| field_error.code).collect();
            }
        }

        status::Custom(
            Status::UnprocessableEntity,
            Json(json!({ "errors": errors })),
        )
        .respond_to(req)
    }
}

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Errors {
                errors: self.errors,
            })
        }
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}

use std::borrow::Borrow;
use std::error::Error as StdError;
use std::io;
use hyper;
use protobuf;
use sawtooth_sdk::signing;

#[derive(Debug)]
pub enum CliError {
//pub struct CliError {
    /// The user has provided invalid inputs; the string by this error
    /// is appropriate for display to the user without additional context
    UserError(String),
    IoError(io::Error),
    SigningError(signing::Error),
    ProtobufError(protobuf::ProtobufError),
    HyperError(hyper::Error),
}

impl StdError for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::UserError(ref s) => &s,
            CliError::IoError(ref err) => err.to_string(),
            CliError::SigningError(ref err) => err.to_string(),
            CliError::ProtobufError(ref err) => err.to_string(),
            CliError::HyperError(ref err) => err.to_string(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            CliError::UserError(ref _s) => None,
            CliError::IoError(ref err) => Some(err.borrow()),
            CliError::SigningError(ref err) => Some(err.borrow()),
            CliError::ProtobufError(ref err) => Some(err.borrow()),
            CliError::HyperError(ref err) => Some(err.borrow()),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::UserError(ref s) => write!(f, "Error: {}", s),
            CliError::IoError(ref err) => write!(f, "IoError: {}", err),
            CliError::SigningError(ref err) => write!(f, "SigningError: {}", err.to_string()),
            CliError::ProtobufError(ref err) => write!(f, "ProtobufError: {}", err.to_string()),
            CliError::HyperError(ref err) => write!(f, "HyperError: {}", err.to_string()),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<protobuf::ProtobufError> for CliError {
    fn from(e: protobuf::ProtobufError) -> Self {
        CliError::ProtobufError(e)
    }
}

impl From<signing::Error> for CliError {
    fn from(e: signing::Error) -> Self {
        CliError::SigningError(e)
    }
}

impl From<hyper::Error> for CliError {
    fn from(e: hyper::Error) -> Self {
        CliError::HyperError(e)
    }
}
