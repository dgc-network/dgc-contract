// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

table! {
    agents (public_key) {
        public_key -> Varchar,
        org_id -> Varchar,
        active -> Bool,
        roles -> Array<Varchar>,
        metadata -> Array<Json>,
    }
}

table! {
    organizations (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Varchar,
    }
}
