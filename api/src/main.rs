// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//#![feature(plugin, decl_macro, custom_derive)]
//#![plugin(rocket_codegen)]

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//extern crate rocket;
extern crate rocket_cors;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate dgc_db;
extern crate sawtooth_sdk;
extern crate protobuf;
extern crate uuid;

extern crate addresser;
#[macro_use] extern crate clap;
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
mod submitBatchList;

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

use std::fs::File;
use std::io::prelude::*;

use sawtooth_sdk::signing;
use sawtooth_sdk::signing::PrivateKey;

use error::CliError;
use key::load_signing_key;
use payload::{
    create_agent_payload,
    create_org_payload,
    update_agent_payload,
    update_org_payload
};
use submit_batch_list::submit_batch_list;

use protos::payload::SmartPayload;
use protos::state::KeyValueEntry;

use protobuf::Message;

fn do_create(
    url: &str,
    private_key: &dyn PrivateKey,
    payload: &SmartPayload,
    output: &str
) -> Result<(), CliError> {

    if !output.is_empty() {
        let mut buffer = File::create(output)?;
        let payload_bytes = payload.write_to_bytes()?;
        buffer.write_all(&payload_bytes).map_err(|err| CliError::IoError(err))?;
        return Ok(())
    }

    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(private_key)?;
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(private_key);

    println!("creating resource {:?}", payload);

    let txn = transaction::create_transaction(&payload, &signer, &public_key.as_hex())?;
    let batch = transaction::create_batch(txn, &signer, &public_key.as_hex())?;
    let batch_list = transaction::create_batch_list_from_one(batch);

    submit_batch_list(
        &format!("{}/batches?wait=120", url),
        &batch_list)
}

#[post("/agent", format = "application/octet-stream", data = "<data>")]
pub fn create_agent(
    //conn: ValidatorConn, 
    data: Vec<u8>
) -> Result<Json<Vec<BatchStatus>>, Custom<Json<JsonValue>>> {

    let url = matches.value_of("url").unwrap_or("http://dgc-api:9001");    
    let key_name = matches.value_of("key");
    let org_id = matches.value_of("org_id").unwrap();
    let public_key = matches.value_of("public_key").unwrap();
    let output = matches.value_of("output").unwrap_or("");
    let roles = matches
        .values_of("roles")
        .unwrap_or(clap::Values::default())
        .map(String::from)
        .collect();
    let metadata_as_strings: Vec<String> = matches
        .values_of("metadata")
        .unwrap_or(clap::Values::default())
        .map(String::from)
        .collect();

    let mut metadata = Vec::<KeyValueEntry>::new();
    for meta in metadata_as_strings {
        let key_val: Vec<&str> = meta.split(",").collect();
        if key_val.len() != 2 {
            return Err(CliError::UserError(
                "Metadata is formated incorrectly".to_string(),
            ));
        }
        let key = match key_val.get(0) {
            Some(key) => key.to_string(),
            None => {
                return Err(CliError::UserError(
                    "Metadata is formated incorrectly".to_string(),
                ))
            }
        };
        let value = match key_val.get(1) {
            Some(value) => value.to_string(),
            None => {
                return Err(CliError::UserError(
                    "Metadata is formated incorrectly".to_string(),
                ))
            }
        };
        let mut entry = KeyValueEntry::new();
        entry.set_key(key);
        entry.set_value(value);
        metadata.push(entry.clone());
    }

    let private_key = load_signing_key(key_name)?;

    //let context = signing::create_context("secp256k1")?;

    let payload = create_agent_payload(org_id, public_key, roles, metadata);
    do_create(&url, &private_key, &payload, &output)?;

    //submit_batches(&mut conn.clone(), &data, 0)
    //    .map_err(map_error)
    //    .and_then(|b| Ok(Json(b)))
}
