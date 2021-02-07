#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use std::env;

// Own code
mod misc;
mod auth;
mod body;
mod storage;
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
        ])
        // Databases
        // .attach(db::Box::fairing())
        // "local" vars
        // .manage(utils::map_generate_users())
        // Config
        .manage(storage::init())
        .manage(auth::OToken(env::var("TOKEN").expect("You must set $TOKEN env var!")))
}
