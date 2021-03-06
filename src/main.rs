#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[cfg(test)] mod tests;

use reqwest::Client;
use std::env;

// Own code
mod misc;
mod auth;
mod body;
mod storage;
mod helpers;
mod subscribe;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![misc::get_index])
        .mount("/subscription", routes![
            subscribe::subscriptions_new,
            subscribe::subscriptions_forward,
        ])
        // All-catchers
        .register(catchers![
            misc::not_found,
            misc::unauth_handler,
            misc::serverside_handler,
        ])
        // Config
        .manage(storage::init_services())
        .manage(storage::init_schemas("./examples/tests"))
        .manage(auth::OToken(env::var("TOKEN").expect("You must set $TOKEN env var!")))
        .manage(Client::new())
}

