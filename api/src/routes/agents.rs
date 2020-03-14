// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crate::auth::Auth;
use crate::config::AppState;
use crate::db::{self, users::UserCreationError};
use crate::errors::{Errors, FieldValidator};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;
use submit::do_create;
use payload::{
    create_agent_payload
//    create_org_payload,
//    update_agent_payload,
//    update_org_payload
};
use protos::state::KeyValueEntry;
use sawtooth_sdk::signing;

#[derive(Deserialize)]
pub struct NewAgent {
    agent: NewAgentData,
}

#[derive(Deserialize, Validate)]
struct NewAgentData {
    //org_id: &str,
    //public_key: &str,
    //roles: Vec<String>,
    //metadata: Vec<KeyValueEntry>,
    //private_key: &str,

    private_key: Option<String>,
    org_id: Option<String>, 
    roles: Option<String>, 
    metadata: Option<String>
/*
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
*/
}

#[post("/agents", format = "json", data = "<new_agent>")]
pub fn post_agents(
    new_agent: Json<NewAgent>,
    //conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_agent = new_agent.into_inner().agent;

    let mut extractor = FieldValidator::validate(&new_agent);
    let org_id = extractor.extract("org_id", new_agent.org_id);
    let roles = extractor.extract("roles", new_agent.roles);
    let metadata = extractor.extract("metadata", new_agent.metadata);
    let mut private_key = extractor.extract("private_key", new_agent.private_key);

    extractor.check()?;

    let url = "http://dgc-api:9001";
    let context = signing::create_context("secp256k1")
        .expect("Error creating the right context");
    //let private_key = context.new_random_private_key()
    //    .expect("Error generating a new Private Key");
    let private_key = hex::decode(private_key);
    let crypto_factory = signing::CryptoFactory::new(context.as_ref());
    let signer = crypto_factory.new_signer(private_key.as_ref());
    let public_key = signer.get_public_key()
        .expect("Error retrieving Public Key")
        .as_hex();

    let payload = create_agent_payload(org_id, public_key, roles, metadata);    
    let output = "";
    do_create(&url, &private_key, &payload, &output);
/*
    db::users::create(&conn, &username, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        })
*/
}

#[derive(Deserialize)]
pub struct LoginAgent {
    agent: LoginAgentData,
}

#[derive(Deserialize)]
struct LoginAgentData {
    email: Option<String>,
    password: Option<String>,
}

#[post("/agents/login", format = "json", data = "<agent>")]
pub fn post_agents_login(
    agent: Json<LoginAgent>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let agent = agent.into_inner().agent;

    let mut extractor = FieldValidator::default();
    let email = extractor.extract("email", agent.email);
    let password = extractor.extract("password", agent.password);
    extractor.check()?;

    db::users::login(&conn, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
        .ok_or_else(|| Errors::new(&[("email or password", "is invalid")]))
}

#[get("/agent")]
pub fn get_agent(auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
    db::users::find(&conn, auth.id).map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
}

#[derive(Deserialize)]
pub struct UpdateAgent {
    agent: db::users::UpdateUserData,
}

#[put("/agent", format = "json", data = "<agent>")]
pub fn put_agent(
    agent: Json<UpdateAgent>,
    auth: Auth,
    conn: db::Conn,
    state: State<AppState>,
) -> Option<JsonValue> {
    db::users::update(&conn, auth.id, &agent.agent)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
}
