pub trait EventHandler<TEvent, TState> {
    fn apply(event: &TEvent, state: Option<TState>) -> Option<TState>;
}

// pub struct EventHandler<TAggregate, TEvent> {
//     handler: fn(aggregate: Option<TAggregate>, event: &TEvent) -> Option<TAggregate>,
// }

// impl<TAggregate, TEvent> EventHandler<TAggregate, TEvent> {
//     pub fn new(handler: fn(Option<TAggregate>, &TEvent) -> Option<TAggregate>) -> Self {
//         EventHandler {
//             handler
//         }
//     }

//     pub fn apply(&self, aggregate: Option<TAggregate>, event: &TEvent) -> Option<TAggregate> {
//         (self.handler)(aggregate, event)
//     }
// }

