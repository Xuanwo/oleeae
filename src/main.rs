#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod route;
mod types;
mod utils;

use route::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![get_packages, get_maintainers, get_builds])
        .attach(utils::RequestIdMiddleware)
        .launch();
}
