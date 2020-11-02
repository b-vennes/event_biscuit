#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod projections;
mod teapot;

use event_biscuit::*;
use rocket_contrib::json::{Json};
use teapot::commands::*;

#[post("/CreateTeapot", format="json", data="<command>")]
fn create_teapot(command: Json<CreateTeapot>) -> Result<String, String> {
    let events = teapot::new().handle(TeapotCommand::CreateTeapot(command.0));

    Ok(serde_json::to_string(&events).expect("Error with event serialization"))
}

#[post("/BrewTea", format="json", data="<command>")]
fn brew_tea(command: Json<BrewTea>) -> Result<String, String> {
    let events = teapot::new().handle(TeapotCommand::BrewTea(command.0));

    Ok(serde_json::to_string(&events).expect("Error with event serialization"))
}

#[post("/PourTea", format="json", data="<command>")]
fn pour_tea(command: Json<PourTea>) -> Result<String, String> {
    let events = teapot::new().handle(TeapotCommand::PourTea(command.0));

    Ok(serde_json::to_string(&events).expect("Error with event serialization"))
}

fn main() {
    rocket::ignite().mount("/", routes![create_teapot, brew_tea, pour_tea]).launch();
}
