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

use error::CliError;
use protobuf::Message;

pub fn submit_batch_list(url: &str, batch_list: &BatchList) -> Result<(), CliError> {
    let hyper_uri = match url.parse::<hyper::Uri>() {
        Ok(uri) => uri,
        Err(e) => return Err(CliError::UserError(format!("Invalid URL: {}: {}", e, url))),
    };

    match hyper_uri.scheme() {
        Some(scheme) => {
            if scheme != "http" {
                return Err(CliError::UserError(format!(
                    "Unsupported scheme ({}) in URL: {}",
                    scheme, url
                )));
            }
        }
        None => {
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
