// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use schema::agents;
use schema::agents::dsl;
use models::{Agent, NewAgent};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryResult;

pub fn create_agent(conn: &PgConnection, agent: NewAgent) -> QueryResult<Agent> {
    diesel::insert_into(agents::table)
        .values(&agent)
        .get_result::<Agent>(conn)
}

pub fn update_agent(conn: &PgConnection, public_key: &str, agent: NewAgent) -> QueryResult<Agent> {
    diesel::update(agents::table)
        .filter(dsl::public_key.eq(public_key))
        .set((
            dsl::public_key.eq(agent.public_key),
            dsl::org_id.eq(agent.org_id),
            dsl::active.eq(agent.active),
            dsl::roles.eq(agent.roles),
            dsl::metadata.eq(agent.metadata)
        ))
        .get_result::<Agent>(conn)
}

pub fn get_agent(conn: &PgConnection, public_key: &str) -> QueryResult<Agent> {
    agents::table
        .select(agents::all_columns)
        .find(public_key)
        .first(conn)
}

pub fn get_agents(conn: &PgConnection) -> QueryResult<Vec<Agent>> {
    agents::table
        .select(agents::all_columns)
        .load(conn)
}
