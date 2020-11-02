use super::command_handler::CommandHandler;
use super::event_handler::EventHandler;

pub struct Aggregate<TAggregate, TCommand, TEvent> {
    command_handler: CommandHandler<TCommand, TEvent>,
    event_handler: EventHandler<TAggregate, TEvent>
}

impl<TAggregate, TCommand, TEvent> Aggregate<TAggregate, TCommand, TEvent> {
    pub fn new(
        command_handler: CommandHandler<TCommand, TEvent>,
        event_handler: EventHandler<TAggregate, TEvent>) -> Aggregate<TAggregate, TCommand, TEvent> {
        Aggregate {
            command_handler,
            event_handler
        }
    }

    pub fn fold(&self, events: Vec<TEvent>) -> Option<TAggregate> {
        events.iter().fold(None, |result, event| {
            self.event_handler.apply(result, event)
        })
    }

    pub fn handle(&self, command: TCommand) -> Result<Vec<TEvent>, String> {
        self.command_handler.handle(command)
    }
}
