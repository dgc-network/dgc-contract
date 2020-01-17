// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use rocket::response::status::Custom;
use guard::validator_conn::ValidatorConn;
use submit::{submit_batches, check_batch_status, BatchStatus};
use submit::TransactionError as error;

use rocket::request::Form;

#[derive(FromForm)]
pub struct TxnQuery {
    wait: u32
}

#[derive(FromForm)]
pub struct StatusQuery {
    wait: Option<u32>,
    ids: String
}

//#[post("/batches?<query>", format = "application/octet-stream", data = "<data>")]
#[post("/batches?<query..>", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns_wait(
        conn: ValidatorConn,
        data: Vec<u8>,
        //query: TxnQuery
        query: Form<TxnQuery>
    ) -> Result<Custom<Json<Vec<BatchStatus>>>, Custom<JsonValue>> {

    let batch_status_list = submit_batches(&mut conn.clone(), &data, query.wait)
        .map_err(map_error)?;

    if batch_status_list
            .iter()
            .all(|x| x.status == "COMMITTED") {

        Ok(Custom(Status::Created, Json(batch_status_list)))
    } else {
        Ok(Custom(Status::Accepted, Json(batch_status_list)))
    }
}

#[post("/batches", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns(
        conn: ValidatorConn, 
        data: Vec<u8>
    ) -> Result<Json<Vec<BatchStatus>>, Custom<JsonValue>> {

    submit_batches(&mut conn.clone(), &data, 0)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}

//#[get("/batch_status?<query>")]
#[get("/batch_status?<query..>")]
pub fn get_batch_status(
        conn: ValidatorConn,
        //query: StatusQuery
        query: Form<StatusQuery>
    ) -> Result<Json<Vec<BatchStatus>>, Custom<JsonValue>> {

    let wait = query.wait.unwrap_or(0);
    let ids: Vec<String> = query.ids
        .split(",")
        .map(String::from)
        .collect();

    check_batch_status(&mut conn.clone(), ids, wait)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}

fn map_error(err: error) -> Custom<JsonValue> {
//    let message = Json(
//        json!({
//            "message": format!("{:?}", err)
//        })
//    );
    let message = json!({
            "message": format!("{:?}", err)
        });

    match err {
        error::BatchParseError(_) |
        error::InvalidBatch(_) |
        error::NoResource(_) |
        error::InvalidId(_) => Custom(Status::BadRequest, message),
        _ => Custom(Status::InternalServerError, message)
    }
}
