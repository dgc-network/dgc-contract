// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![recursion_limit="128"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate postgres;

use diesel::result::Error;

mod schema;
mod agents_helper;
mod orgs_helper;

pub mod pools;
pub mod models;

pub use orgs_helper::*;
pub use agents_helper::*;

pub use Error::NotFound;
pub use diesel::pg::PgConnection;
pub use r2d2_diesel::ConnectionManager;
pub use r2d2::PooledConnection;

pub type QueryError = Error;
