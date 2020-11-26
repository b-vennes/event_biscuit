pub mod state;
pub mod commands;
pub mod events;

use uuid::Uuid;
use state::TeapotState;
use commands::*;
use events::*;
use event_biscuit::aggregate::Aggregate;
use event_biscuit::event_handler::EventHandler;
use event_biscuit::command_handler::CommandHandler;

pub struct TeapotAggregate {}

impl EventHandler<TeapotEvent, TeapotState> for TeapotAggregate {
    fn apply(event: &TeapotEvent, state: Option<TeapotState>) -> Option<TeapotState> {
        match event {
            TeapotEvent::Created(id, data) => Some(TeapotState {
                id: *id,
                name: data.name.clone(),
                size: data.size,
                remaining: data.size,
            }),
            TeapotEvent::TeaBrewed(_) => match state {
                Some(state) => Some(TeapotState { remaining: state.size, .. state }),
                None => None
            },
            TeapotEvent::TeaPoured(_) => match state {
                Some(state) => Some(TeapotState { remaining: state.remaining - 1, .. state }),
                None => None
            }
        }
    }
}

impl CommandHandler<TeapotCommand, TeapotEvent, TeapotState> for TeapotAggregate {
    fn validate(command: &TeapotCommand, state: Option<TeapotState>) -> Option<String> {
        match command {
            TeapotCommand::BrewTea(_) => match state {
                Some(_) => None,
                None => Some("Teapot does not exist".to_string())
            },
            TeapotCommand::CreateTeapot(_) => None,
            TeapotCommand::PourTea(_) => match state {
                Some(state) => if state.remaining > 0 { 
                    None 
                } else {
                    Some("No tea remaining in teapot.".to_string())
                },
                None => Some("Teapot does not exist".to_string())
            }
        }
    }

    fn to_events(command: TeapotCommand) -> Vec<TeapotEvent> {
        match command {
            TeapotCommand::CreateTeapot(data) => vec![
                TeapotEvent::Created(
                    Uuid::new_v4(), 
                    TeapotCreatedData {
                        name: data.name,
                        size: data.size,
                    })],
            TeapotCommand::BrewTea(id) => vec![TeapotEvent::TeaBrewed(id)],
            TeapotCommand::PourTea(id) => vec![TeapotEvent::TeaPoured(id)]
        }
    }
}
