// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crate::db;
use rocket_contrib::json::JsonValue;

#[get("/tags")]
pub fn get_tags(conn: db::Conn) -> JsonValue {
    json!({ "tags": db::articles::tags(&conn) })
}
