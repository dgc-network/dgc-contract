// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate validator_derive;
//#[macro_use] extern crate cfg_if;
//#[macro_use] extern crate clap;
//#[macro_use] extern crate log;

extern crate rocket_cors;
extern crate serde;
extern crate jsonwebtoken as jwt;
extern crate dotenv;
extern crate validator;
extern crate rand;
extern crate slug;
extern crate crypto;
extern crate chrono;
extern crate sawtooth_sdk;
extern crate protobuf;
extern crate uuid;
extern crate hyper;
extern crate addresser;
extern crate futures;
extern crate tokio_core;
extern crate reqwest;
extern crate tokio;

mod auth;
mod config;
mod error;
mod errors;
mod schema;
mod payload;
mod protos;
//mod submit;
mod transaction;
//pub mod handler;

mod db;
mod models;
mod routes;

use dotenv::dotenv;
use rocket_contrib::json::JsonValue;
use rocket_cors::Cors;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}

//pub fn rocket() -> rocket::Rocket {
fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api",
            routes![
                routes::agents::post_agents,
                routes::agents::post_agents_login,
                routes::agents::put_agent,
                routes::agents::get_agent,
                routes::users::post_users,
                routes::users::post_users_login,
                routes::users::put_user,
                routes::users::get_user,
                routes::articles::post_articles,
                routes::articles::put_articles,
                routes::articles::get_article,
                routes::articles::delete_article,
                routes::articles::favorite_article,
                routes::articles::unfavorite_article,
                routes::articles::get_articles,
                routes::articles::get_articles_feed,
                routes::articles::post_comment,
                routes::articles::get_comments,
                routes::articles::delete_comment,
                routes::tags::get_tags,
                routes::profiles::get_profile,
                routes::profiles::follow,
                routes::profiles::unfollow,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
