// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate rocket;

use rocket_contrib::json::Json;
use guard::db_conn::DbConn;

use pike_db as db;
use pike_db::models::Agent;

#[get("/agent/<publickey>")]
fn get_agent(conn: DbConn, publickey: String) -> Option<Json<Agent>> {
    if let Ok(agent) = db::get_agent(&conn, &publickey) {
        Some(Json(agent))
    } else {
        None
    }
}

#[get("/agent")]
fn get_agents(conn: DbConn) -> Json<Vec<Agent>> {
    if let Ok(agents) = db::get_agents(&conn) {
        Json(agents)
    } else {
        Json(vec![])
    }
}
