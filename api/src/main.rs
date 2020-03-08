// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
//extern crate rocket;
extern crate rocket_cors;
extern crate serde_yaml;
extern crate serde_json;
extern crate dgc_db;
extern crate sawtooth_sdk;
extern crate protobuf;
extern crate uuid;

extern crate addresser;
//#[macro_use] extern crate clap;
extern crate crypto;
extern crate futures;
extern crate hyper;
//extern crate protobuf;
//extern crate sawtooth_sdk;
extern crate tokio_core;
extern crate users;

mod error;
mod key;
mod payload;
mod protos;
mod transaction;
mod do_create;

mod openapi;
mod routes;
mod guard;
mod submit;
#[cfg(test)] mod tests;

use std::env;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders, Error};
use rocket_contrib::json::{Json, JsonValue};
use routes::{agents, organizations};
use dgc_db::pools;
use routes::transactions;

use sawtooth_sdk::messaging::zmq_stream::ZmqMessageConnection;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![
            agents::create_agent,
            agents::get_agent,
            agents::get_agents,
            organizations::get_org,
            organizations::get_orgs,
            hello])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
/*
//#[error(404)]
#[catch(404)]
fn not_found(_: &rocket::Request) -> Json<JsonValue> {
    Json(json!({
        "message": "Not Found"
    }))
}

//#[error(500)]
#[catch(500)]
fn internal_server_error(_: &rocket::Request) -> Json<JsonValue> {
    Json(json!({
        "message": "Internal Server Error"
    }))
}

fn main() -> Result<(), Error> {
    //let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:9002"]);
    //assert!(failed_origins.is_empty());

    //let options = rocket_cors::Cors {
    let options = rocket_cors::CorsOptions {
        //allowed_origins: allowed_origins,
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let database_url = if let Ok(s) = env::var("DATABASE_URL") {
        s
    } else {
        "postgres://localhost:5432".into()
    };

    let validator_url = if let Ok(s) = env::var("VALIDATOR_URL") {
       s
    } else {
        "tcp://localhost:8004".into()
    };

    rocket::ignite()
        .mount("/", routes![
               hello,
               openapi::openapi_json,
               openapi::openapi_yaml,
               agents::create_agent,
               agents::get_agent,
               agents::get_agents,
               organizations::get_org,
               organizations::get_orgs,
               transactions::submit_txns,
               transactions::submit_txns_wait,
               transactions::get_batch_status])
        .manage(pools::init_pg_pool(database_url))
        .manage(ZmqMessageConnection::new(&validator_url))
        .attach(options)
        //.catch(errors![not_found, internal_server_error])
        .register(catchers![not_found, internal_server_error])
        .launch();

    Ok(())
}

//extern crate protobuf;

//mod trans;
//mod protos;

*/