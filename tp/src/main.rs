// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate cfg_if;
extern crate protobuf;
extern crate crypto;
extern crate addresser;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate sabre_sdk;
    } else {
        #[macro_use]
        extern crate clap;
        #[macro_use]
        extern crate log;
        extern crate sawtooth_sdk;
        extern crate simple_logger;

        use log::LogLevel;
        use sawtooth_sdk::processor::TransactionProcessor;
        use handler::SmartTransactionHandler;
    }
}

pub mod handler;
mod protos;

//use sawtooth_sdk::processor::TransactionProcessor;
//use handler::SmartTransactionHandler;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let matches = clap_app!(wasm_store_tp =>
        (version: crate_version!())
        (about: "Implements the dgc transaction family")
        (@arg connect: -C --connect +takes_value
         "connection endpoint for validator")
        (@arg verbose: -v --verbose +multiple
         "increase output verbosity"))
        .get_matches();

    let logger = match matches.occurrences_of("verbose") {
        1 => simple_logger::init_with_level(LogLevel::Info),
        2 => simple_logger::init_with_level(LogLevel::Debug),
        0 | _ => simple_logger::init_with_level(LogLevel::Warn),
    };

    logger.expect("Failed to create logger");

    let connect = matches
        .value_of("connect")
        .unwrap_or("tcp://localhost:4004");

    let handler = SmartTransactionHandler::new();
    let mut processor = TransactionProcessor::new(connect);

    processor.add_handler(&handler);
    processor.start();
}

#[cfg(target_arch = "wasm32")]
fn main() {
}
