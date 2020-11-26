#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod projections;
mod teapot;

use event_biscuit::aggregate::Aggregate;
use rocket_contrib::json::{Json};
use teapot::commands::{TeapotCommand, CreateTeapotData};
use teapot::TeapotAggregate;
use uuid::Uuid;

#[post("/CreateTeapot", format="json", data="<data>")]
fn create_teapot(data: Json<CreateTeapotData>) -> Result<String, String> {
    TeapotAggregate::handle(TeapotCommand::CreateTeapot(data.0), vec![]);

    Ok("test".to_string())
}

#[post("/BrewTea/<id>")]
fn brew_tea(id: String) -> Result<String, String> {
    Ok("".to_string())
}

#[post("/PourTea/<id>")]
fn pour_tea(id: String) -> Result<String, String> {
    Ok("".to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![create_teapot, brew_tea, pour_tea]).launch();
}
