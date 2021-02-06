#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// Third Party
// use rocket_contrib::templates::Template;
// use rocket_contrib::serve::StaticFiles;

// Own code
mod misc;
mod auth;
mod storage;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        // Home page
        .mount("/", routes![misc::get_index])
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
}
