// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//! Contains functions which assist with batch submission to a REST API

use hyper;
use hyper::Method;
use hyper::client::{Client, Request};
use std::str;
use hyper::header::{ContentLength, ContentType};
use futures::{future, Future};
use futures::Stream;
use tokio_core;

use sawtooth_sdk::messages::batch::BatchList;

//use errors::Errors;
use protobuf::Message;

//use std::fs::File;
//use std::io::prelude::*;

use sawtooth_sdk::signing;
use sawtooth_sdk::signing::PrivateKey;

use error::CliError;
//use key::load_signing_key;
//use submit_batch_list::submit_batch_list;

use protos::payload::SmartPayload;
//use protos::state::KeyValueEntry;

//use protobuf::Message;

use transaction;
/*
pub fn do_create(
    //url: &str,
    private_key: &dyn PrivateKey,
    payload: &SmartPayload,
    //output: &str
) -> Result<(), CliError> {
//) -> Result<(), Errors> {

    //if !output.is_empty() {
    //    let mut buffer = File::create(output)?;
    //    let payload_bytes = payload.write_to_bytes()?;
    //    buffer.write_all(&payload_bytes).map_err(|err| CliError::IoError(err))?;
        //buffer.write_all(&payload_bytes).map_err(|err| Errors::new(&[("IoError:", format!("{}", err))]))?;
    //    return Ok(())
    //}

    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(private_key)?;
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(private_key);

    println!("creating resource {:?}", payload);

    let txn = transaction::create_transaction(&payload, &signer, &public_key.as_hex())?;
    let batch = transaction::create_batch(txn, &signer, &public_key.as_hex())?;
    let batch_list = transaction::create_batch_list_from_one(batch);

    let url = "http://dgc-api:9001";
    submit_batch_list(
        &format!("{}/batches?wait=120", url),
        &batch_list)
}
*/
pub fn submit_batch_list(
    url: &str, 
    batch_list: &BatchList
) -> Result<(), CliError> {
//) -> Result<(), Errors> {
    let hyper_uri = match url.parse::<hyper::Uri>() {
        Ok(uri) => uri,
        //Err(e) => Errors::new(&[("Invalid URL:", format!(
        //    "{}: {}", e, url
        //))]),
        Err(e) => return Err(CliError::UserError(format!("Invalid URL: {}: {}", e, url))),
    };

    match hyper_uri.scheme() {
        Some(scheme) => {
            if scheme != "http" {
                //Errors::new(&[("Unsupported scheme", format!(
                //    "({}) in URL: {}", scheme, url
                //))]);
                return Err(CliError::UserError(format!(
                    "Unsupported scheme ({}) in URL: {}",
                    scheme, url
                )));
            }
        }
        None => {
            //Errors::new(&[("No scheme", format!("in URL: {}", url))]);
            return Err(CliError::UserError(format!("No scheme in URL: {}", url)));
        }
    }

    let mut core = tokio_core::reactor::Core::new()?;
    let handle = core.handle();
    let client = Client::configure().build(&handle);

    let bytes = batch_list.write_to_bytes()?;

    let mut req = Request::new(Method::Post, hyper_uri);
    req.headers_mut().set(ContentType::octet_stream());
    req.headers_mut().set(ContentLength(bytes.len() as u64));
    req.set_body(bytes);

    let work = client.request(req).and_then(|res| {
        res.body()
            .fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, hyper::Error>(v)
            })
            .and_then(move |chunks| {
                let body = String::from_utf8(chunks).unwrap();
                future::ok(body)
            })
    });

    let body = core.run(work)?;
    println!("Response Body:\n{}", body);

    Ok(())
}

