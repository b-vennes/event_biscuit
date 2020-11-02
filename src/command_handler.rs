/// Validates and handles a command of type TCommand.
pub struct CommandHandler<TCommand, TEvent> {
    validator: fn(&TCommand) -> Option<String>,
    handler: fn(TCommand) -> Vec<TEvent>
}

/// Implementation of CommandHandler.
impl<TCommand, TEvent> CommandHandler<TCommand, TEvent> {
    
    /// Creates a new command handler.
    pub fn new(
        validator: fn(&TCommand) -> Option<String>,
        handler: fn(TCommand) -> Vec<TEvent>) -> Self {
        CommandHandler {
            validator,
            handler
        }
    }

    /// Validates and handles a command.
    /// Returns a result of either a list of resulting events or a validation error message.
    pub fn handle(&self, command: TCommand) -> Result<Vec<TEvent>, String> {
        match (self.validator)(&command) {
            Some(error) => Err(error),
            None => Ok((self.handler)(command))
        }
    }
}
