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

#[derive(Deserialize)]
pub struct NewAgent {
    agent: NewAgentData,
}

#[derive(Deserialize, Validate)]
struct NewAgentData {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

#[post("/agents", format = "json", data = "<new_agent>")]
pub fn post_agents(
    new_agent: Json<NewAgent>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_agent = new_agent.into_inner().agent;

    let mut extractor = FieldValidator::validate(&new_agent);
    let username = extractor.extract("username", new_agent.username);
    let email = extractor.extract("email", new_agent.email);
    let password = extractor.extract("password", new_agent.password);

    extractor.check()?;

    db::users::create(&conn, &username, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        })
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
    user: Json<LoginAgent>,
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
    user: db::users::UpdateUserData,
}

#[put("/agent", format = "json", data = "<agent>")]
pub fn put_agent(
    user: Json<UpdateAgent>,
    auth: Auth,
    conn: db::Conn,
    state: State<AppState>,
) -> Option<JsonValue> {
    db::users::update(&conn, auth.id, &user.user)
        .map(|user| json!({ "user": user.to_user_auth(&state.secret) }))
}
