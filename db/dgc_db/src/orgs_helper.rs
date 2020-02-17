// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use schema::organizations;
use schema::organizations::dsl;
use models::{NewOrganization, Organization};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryResult;

pub fn create_organization(
    conn: &PgConnection, 
    org: NewOrganization
) -> QueryResult<Organization> {
    diesel::insert_into(organizations::table)
        .values(&org)
        .get_result::<Organization>(conn)
}

pub fn update_organization(
    conn: &PgConnection, 
    id: &str, org: NewOrganization
) -> QueryResult<Organization> {
    diesel::update(organizations::table)
        .filter(dsl::id.eq(id))
        .set((
            dsl::name.eq(org.name),
            dsl::address.eq(org.address),
        ))
        .get_result::<Organization>(conn)
}

pub fn get_org(
    conn: &PgConnection, 
    id: &str
) -> QueryResult<Organization> {
    organizations::table
        .select(organizations::all_columns)
        .find(id)
        .first(conn)
}

pub fn get_orgs(
    conn: &PgConnection
) -> QueryResult<Vec<Organization>> {
    organizations::table
        .select(organizations::all_columns)
        .load(conn)
}
