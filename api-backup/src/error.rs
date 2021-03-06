// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//use std::borrow::Borrow;

//use std;
//use std::error::Error as StdError;
use std::io;

use hyper;

use protobuf;

use sawtooth_sdk::signing;

#[derive(Debug)]
pub enum CliError {
    /// The user has provided invalid inputs; the string by this error
    /// is appropriate for display to the user without additional context
    UserError(String),
    IoError(io::Error),
    SigningError(signing::Error),
    ProtobufError(protobuf::ProtobufError),
    HyperError(hyper::Error),
}
/*
impl StdError for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::UserError(ref s) => &s,
            CliError::IoError(ref err) => err.description(),
            CliError::SigningError(ref err) => err.description(),
            CliError::ProtobufError(ref err) => err.description(),
            CliError::HyperError(ref err) => err.description(),
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
            CliError::SigningError(ref err) => write!(f, "SigningError: {}", err.description()),
            CliError::ProtobufError(ref err) => write!(f, "ProtobufError: {}", err.description()),
            CliError::HyperError(ref err) => write!(f, "HyperError: {}", err.description()),
        }
    }
}
*/
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
