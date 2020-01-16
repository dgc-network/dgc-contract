/* 
* Copyright (c) The dgc.network
* SPDX-License-Identifier: Apache-2.0
*/
CREATE TABLE agents (
    public_key VARCHAR(70) PRIMARY KEY NOT NULL,
    org_id VARCHAR(256) NOT NULL,
    active BOOLEAN NOT NULL,
    roles VARCHAR(256) [] NOT NULL,
    metadata JSON [] NOT NULL
);

CREATE TABLE organizations (
    id VARCHAR(256) PRIMARY KEY NOT NULL,
    name VARCHAR(256) NOT NULL,
    address VARCHAR(256) NOT NULL
);
