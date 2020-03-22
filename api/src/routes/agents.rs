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
//use protos::state::KeyValueEntry;
use protos::state::{
    KeyValueEntry,
    Agent, AgentList, 
    Organization, OrganizationList
};
use addresser::{resource_to_byte, Resource};
use sawtooth_sdk::signing;
use crypto::sha2::Sha512;

const NAMESPACE: &'static str = "cad11d";

#[derive(Deserialize)]
pub struct NewAgent {
    agent: NewAgentData,
}

#[derive(Deserialize, Validate)]
struct NewAgentData {
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
    let roles_as_strings = extractor.extract("roles", new_agent.roles);
    let metadata_as_strings = extractor.extract("metadata", new_agent.metadata);
    let private_key_hex_string = extractor.extract("private_key", new_agent.private_key);

    extractor.check()?;

    let url = "http://dgc-api:9001";
    let context = signing::create_context("secp256k1")
        .expect("Error creating the right context");
    //let private_key = context.new_random_private_key()
    //    .expect("Error generating a new Private Key");
    let private_key = signing::secp256k1::Secp256k1PrivateKey::from_hex(&private_key_hex_string)
        .expect("Error retrieving Private Key");
    let crypto_factory = signing::CryptoFactory::new(context.as_ref());
    //let signer = crypto_factory.new_signer(&private_key.as_ref());
    let signer = crypto_factory.new_signer(&private_key);
    let public_key = signer.get_public_key()
        .expect("Error retrieving Public Key")
        .as_hex();

    let mut roles = Vec::<String>::new();
    for role in roles_as_strings.chars() {
        let entry: String = role.to_string().split(",").collect();
        roles.push(entry.clone());
    }

    let mut metadata = Vec::<KeyValueEntry>::new();
    for meta in metadata_as_strings.chars() {
        let meta_as_string = meta.to_string();
        let key_val: Vec<&str> = meta_as_string.split(",").collect();
        if key_val.len() != 2 {
            "Metadata is formated incorrectly".to_string();            
        }
        let key = match key_val.get(0) {
            Some(key) => key.to_string(),
            None => "Metadata is formated incorrectly".to_string()
        };
        let value = match key_val.get(1) {
            Some(value) => value.to_string(),
            None => "Metadata is formated incorrectly".to_string()
        };
        let mut entry = KeyValueEntry::new();
        entry.set_key(key);
        entry.set_value(value);
        metadata.push(entry.clone());
    }

    let payload = create_agent_payload(&org_id, &public_key, roles, metadata);    
    let output = "";
    do_create(&url, &private_key, &payload, &output);
    Ok(json!({ "createAgent": "done" }))

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


//pub fn get_agent(auth: Auth, conn: db::Conn, state: State<AppState>) -> Option<JsonValue> {
//    db::users::find(&conn, auth.id).map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
//}

fn compute_address(name: &str, resource: Resource) -> String {
    let mut sha = Sha512::new();
    sha.input(name.as_bytes());

    String::from(NAMESPACE) + &resource_to_byte(resource) + &sha.result_str()[..62].to_string()
}

pub struct SmartState<'a> {
    context: &'a mut dyn TransactionContext,
}

impl<'a> SmartState<'a> {
    pub fn new(context: &'a mut dyn TransactionContext) -> SmartState {
        SmartState { context: context }
    }
    
    #[get("/agent")]
    pub fn get_agent(
        &mut self, 
        public_key: &str
    ) -> Result<Option<Agent>, ApplyError> {
        let address = compute_address(public_key, Resource::AGENT);
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let agents: AgentList = match protobuf::parse_from_bytes(packed.as_slice()) {
                    Ok(agents) => agents,
                    Err(err) => {
                        return Err(ApplyError::InternalError(format!(
                            "Cannot deserialize record container: {:?}",
                            err,
                        )))
                    }
                };
    
                for agent in agents.get_agents() {
                    if agent.public_key == public_key {
                        return Ok(Some(agent.clone()));
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }    
}