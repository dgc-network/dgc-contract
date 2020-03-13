// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use sawtooth_sdk::messaging::zmq_stream::{ZmqMessageSender, ZmqMessageConnection};
use sawtooth_sdk::messaging::stream::MessageConnection;

pub struct ValidatorConn(pub ZmqMessageSender);

impl<'a, 'r> FromRequest<'a, 'r> for ValidatorConn {

    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ValidatorConn, ()> {
        let connection = request.guard::<State<ZmqMessageConnection>>()?;
        let (sender, _) = connection.create();
        Outcome::Success(ValidatorConn(sender))
    }
}

impl Deref for ValidatorConn {
    type Target = ZmqMessageSender;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
