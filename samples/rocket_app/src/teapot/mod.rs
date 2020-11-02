pub mod aggregate;
pub mod commands;
pub mod events;

use uuid::Uuid;
use aggregate::*;
use commands::*;
use events::*;
use event_biscuit::aggregate::*;
use event_biscuit::event_handler::*;
use event_biscuit::command_handler::*;

fn handle_create_teapot(command: CreateTeapot) -> Vec<TeapotEvent> {
    vec![
        TeapotEvent::Created(
            TeapotCreated {
                id: Uuid::new_v4(),
                name: command.name,
                size: command.size
            }
        )
    ]
}

fn validate_create_teapot(command: &CreateTeapot) -> Option<String> {
    None
}

fn handle_brew_tea(command: BrewTea) -> Vec<TeapotEvent> {
    vec![
        TeapotEvent::TeaBrewed(
            TeaBrewed {
                id: command.id
            }
        )
    ]
}

fn validate_brew_tea(command: &BrewTea) -> Option<String> {
    None
}

fn handle_pour_tea(command: PourTea) -> Vec<TeapotEvent> {
    vec![
        TeapotEvent::TeaPoured(
            TeaPoured {
                id: command.id
            }
        )
    ]
}

fn validate_pour_tea(command: &PourTea) -> Option<String> {
    None
}

fn get_command_handler() -> CommandHandler<TeapotCommand, TeapotEvent> {
    let validator: fn(&TeapotCommand) -> Option<String> = |command: &TeapotCommand| match command {
        TeapotCommand::CreateTeapot(create) => validate_create_teapot(create),
        TeapotCommand::BrewTea(brew) => validate_brew_tea(brew),
        TeapotCommand::PourTea(pour) => validate_pour_tea(pour)
    };

    let command_handler = |command: TeapotCommand| match command {
        TeapotCommand::CreateTeapot(create) => handle_create_teapot(create),
        TeapotCommand::BrewTea(brew) => handle_brew_tea(brew),
        TeapotCommand::PourTea(pour) => handle_pour_tea(pour)
    };

    CommandHandler::new(validator, command_handler)
}

fn handle_event(aggregate: Option<TeapotAggregate>, event: &TeapotEvent) -> Option<TeapotAggregate> {
    aggregate
}

fn get_event_handler() -> EventHandler<TeapotAggregate, TeapotEvent> {
    EventHandler::new(handle_event)
}

pub fn new() -> Aggregate<TeapotAggregate, TeapotCommand, TeapotEvent> {
    Aggregate::new(
        get_command_handler(),
        get_event_handler()
    )
}
