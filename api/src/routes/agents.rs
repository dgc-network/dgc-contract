// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//extern crate rocket;

use rocket_contrib::json::Json;
use guard::db_conn::DbConn;

use dgc_db as db;
use dgc_db::models::Agent;

#[get("/agent/<publickey>")]
pub fn get_agent(conn: DbConn, publickey: String) -> Option<Json<Agent>> {
    if let Ok(agent) = db::get_agent(&conn, &publickey) {
        Some(Json(agent))
    } else {
        None
    }
}

#[get("/agent")]
pub fn get_agents(conn: DbConn) -> Json<Vec<Agent>> {
    if let Ok(agents) = db::get_agents(&conn) {
        Json(agents)
    } else {
        Json(vec![])
    }
}

//use rocket_contrib::json::{Json, JsonValue};
//use rocket::http::Status;
//use rocket::response::status::Custom;
//use guard::validator_conn::ValidatorConn;
//use submit::{submit_batches, check_batch_status, BatchStatus};
use submit::TransactionError as error;
use rocket::request::Form;
use error::CliError;
use do_create::do_create;
use payload::{
    create_agent_payload,
    create_org_payload,
    update_agent_payload,
    update_org_payload
};
use sawtooth_sdk::signing;
//use sawtooth_sdk::signing::PrivateKey;
//use key::load_signing_key;
use protos::state::KeyValueEntry;
use rocket::request::{FromForm, FormItems};

//#[derive(FromForm)]
struct Item { 
    private_key: String, 
    org_id: String, 
    roles: Vec<&str>, 
    metadata: Vec<KeyValueEntry>
}

impl<'f> FromForm<'f> for Item {
    type Error = String;

    fn from_form(form_items: &mut FormItems<'f>, strict: bool) -> Result<Self, String> {
        let mut private_key = None;
        let mut org_id = None;
        let mut roles = None;
        let mut metadata = None;

        for form_item in form_items {
            let (key, value) = form_item.key_value();
            // Note: we explicitly decode in the match arms to save work rather
            // than decoding every form item blindly.
            match key.as_str() {
                "private_key" => {
                    if private_key.is_some() {
                        return Err("private_key parameter must not occur more than once".to_owned());
                    } else {
                        match value.url_decode() {
                            Ok(v) => private_key = Some(v),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                "org_id" => {
                    if org_id.is_some() {
                        return Err("org_id parameter must not occur more than once".to_owned());
                    } else {
                        match value.url_decode() {
                            Ok(v) => org_id = Some(v),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                "roles" => {
                    if roles.is_some() {
                        return Err("roles parameter must not occur more than once".to_owned());
                    } else {
                        match value.url_decode() {
                            Ok(v) => roles = Some(v),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                "metadata" => {
                    if metadata.is_some() {
                        return Err("metadata parameter must not occur more than once".to_owned());
                    } else {
                        match value.url_decode() {
                            Ok(v) => metadata = Some(v),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
/*                
                "roles" => {
                    if roles.is_some() {
                        return Err("roles parameter must not occur more than once".to_owned());
                    } else {
                        let decoded;
                        match value.url_decode() {
                            Ok(v) => decoded = v,
                            Err(e) => return Err(e.to_string()),
                        }
                        roles = Some(
                            serde_json::from_str::<InputValue<_>>(&decoded)
                                .map_err(|err| err.to_string())?,
                        );
                    }
                }
*/                
                _ => {
                    if strict {
                        return Err(format!("Prohibited extra field '{}'", key).to_owned());
                    }
                }
            }
        }
/*
        if let Some(query) = query {
            Ok(GraphQLRequest(GraphQLBatchRequest::Single(
                http::GraphQLRequest::new(query, operation_name, variables),
            )))
        } else {
            Err("Query parameter missing".to_owned())
        }
*/        
    }
}
/*
impl<'f> FromForm<'f> for Item {
    // In practice, we'd use a more descriptive error type.
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Item, ()> {
        let mut field = None;

        for item in items {
            match item.key.as_str() {
                "balloon" | "space" if field.is_none() => {
                    let decoded = item.value.url_decode().map_err(|_| ())?;
                    field = Some(decoded);
                }
                _ if strict => return Err(()),
                _ => { /* allow extra value when not strict */ }
            }
        }

        field.map(|field| Item { field }).ok_or(())
    }
}
*/
#[post("/agent", format = "application/octet-stream", data = "<input>")]
pub fn create_agent(input: Form<Item>) -> String {
    if input.private_key.is_empty() {
        "PrivateKey cannot be empty.".to_string()
    } else {
        let url = "http://dgc-api:9001";
        let output = "";

        let private_key = input.private_key;
        let org_id = input.org_id;
        let roles = input.roles;

        let metadata_as_strings = input.metadata;
        let mut metadata = Vec::<KeyValueEntry>::new();
/*
        let strings = "bananas,apples,pear".split(",");
        for s in strings {
            println!("{}", s)
        }
        let strings: Vec<&str> = "bananas,apples,pear".split(",").collect(); // ["bananas", "apples", "pear"]

        let mut split = "some string 123 ffd".split("123");
        for s in split {
            println!("{}", s)
        }
        let vec = split.collect::<Vec<&str>>();
*/
        for meta in metadata_as_strings.chars() {
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
    
        let context = signing::create_context("secp256k1");
        let public_key = context.get_public_key(private_key);
    
        let payload = create_agent_payload(&org_id, public_key, roles, metadata);
    
        do_create(&url, &private_key, &payload, &output);
    
        "Data added.".to_string()
    }
}
/*
pub fn create_agent(
    //conn: ValidatorConn, 
    //data: Vec<u8>
) -> Result<Json<Vec<BatchStatus>>, Custom<Json<JsonValue>>> {
/*
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
*/
    let private_key = load_signing_key(key_name)?;

    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(private_key)?;

    let payload = create_agent_payload(org_id, public_key, roles, metadata);

    do_create(&url, &private_key, &payload, &output)?;

    //submit_batches(&mut conn.clone(), &data, 0)
    //    .map_err(map_error)
    //    .and_then(|b| Ok(Json(b)))
}
*/