use crate::command_handler::CommandHandler;
use crate::event_handler::EventHandler;

pub trait Aggregate<TCommand, TEvent, TState>: CommandHandler<TCommand, TEvent, TState> + EventHandler<TEvent, TState>
{
    fn handle(command: TCommand, events: Vec<TEvent>) -> Result<Vec<TEvent>, String> {
        let state = events.iter().fold(None, |acc: Option<TState>, event: &TEvent| {
            Self::apply(event, acc)
        });

        if let Some(error) = Self::validate(&command, state) {
            Err(error)
        } else {
            Ok(Self::to_events(command))
        }
    }
}

impl<TCommand, TEvent, TState, T: EventHandler<TEvent, TState> + CommandHandler<TCommand, TEvent, TState>> Aggregate<TCommand, TEvent, TState> for T {}

// pub struct Aggregate<TAggregate, TCommand, TEvent> {
//     command_handler: CommandHandler<TCommand, TEvent>,
//     event_handler: EventHandler<TAggregate, TEvent>
// }

// impl<TAggregate, TCommand, TEvent> Aggregate<TAggregate, TCommand, TEvent> {
//     pub fn new(
//         command_handler: CommandHandler<TCommand, TEvent>,
//         event_handler: EventHandler<TAggregate, TEvent>) -> Aggregate<TAggregate, TCommand, TEvent> {
//         Aggregate {
//             command_handler,
//             event_handler
//         }
//     }

//     pub fn fold(&self, events: Vec<TEvent>) -> Option<TAggregate> {
//         events.iter().fold(None, |result, event| {
//             self.event_handler.apply(result, event)
//         })
//     }

//     pub fn handle(&self, command: TCommand) -> Result<Vec<TEvent>, String> {
//         self.command_handler.handle(command)
//     }
// }
