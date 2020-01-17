// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate rocket;
extern crate serde_yaml;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

use serde_json::Value;

const SWAGGER_FILENAME: &'static str = "openapi.yaml";

#[get("/openapi.json")]
pub fn openapi_json() -> String {
    let mut file = File::open(SWAGGER_FILENAME).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let deserialized_map: Value = serde_yaml::from_str(&contents).unwrap();
    let j = serde_json::to_string_pretty(&deserialized_map).unwrap();

    return j
}

#[get("/openapi.yaml")]
pub fn openapi_yaml() -> String {
    let mut file = File::open(SWAGGER_FILENAME).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents;
}
