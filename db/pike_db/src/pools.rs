// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pg_pool(db_url: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool")
}
