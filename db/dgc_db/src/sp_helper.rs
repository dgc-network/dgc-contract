// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use schema::smartpermissions;
use schema::smartpermissions::dsl;
use models::{SmartPermission, NewSmartPermission};

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryResult;

pub fn create_smart_permission(
    conn: &PgConnection, 
    sp: NewSmartPermission
) -> QueryResult<SmartPermission> {
    diesel::insert_into(smartpermissions::table)
        .values(&sp)
        .get_result::<SmartPermission>(conn)
}

pub fn delete_smart_permission(
    conn: &PgConnection, 
    address: &str
) -> QueryResult<SmartPermission> {
    diesel::delete(smartpermissions::table)
        .filter(dsl::address.eq(address))
        .get_result::<SmartPermission>(conn)
}

pub fn get_smart_permission(
    conn: &PgConnection, 
    name: &str
) -> QueryResult<SmartPermission> {
    smartpermissions::table
        .select(smartpermissions::all_columns)
        .find(name)
        .first(conn)
}

pub fn get_smart_permissions(
    conn: &PgConnection
) -> QueryResult<Vec<SmartPermission>> {
    smartpermissions::table
        .select(smartpermissions::all_columns)
        .load(conn)
}
