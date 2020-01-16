// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate protoc_rust;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    fs::create_dir_all("src/protos").unwrap();
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &[
            "../../../protos/payload.proto",
            "../../../protos/agent.proto",
            "../../../protos/property.proto",
            "../../../protos/proposal.proto",
            "../../../protos/record.proto"
        ],
        includes: &["../../../protos"],
    }).expect("protoc");

    let mut file = File::create("src/protos/mod.rs").unwrap();
    file.write_all(b"pub mod payload;\n").unwrap();
    file.write_all(b"pub mod agent;\n").unwrap();
    file.write_all(b"pub mod property;\n").unwrap();
    file.write_all(b"pub mod proposal;\n").unwrap();
    file.write_all(b"pub mod record;\n").unwrap();
}
