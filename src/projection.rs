pub trait Projection<A, TQuery, TCommand, TEvent, TState> where A: EventHandler<TEvent, TState> {
    fn query(query: TQuery) -> Option<TState>;
}
