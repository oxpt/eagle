use eagle_types::messages::ClientCommandIndex;

pub trait Repository<T: 'static> {
    fn push_command(&mut self, command: T) -> ClientCommandIndex;
    fn ack(&mut self, index: ClientCommandIndex);
    fn pending_commands(&self) -> Vec<(ClientCommandIndex, T)>;
}
