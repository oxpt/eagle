use eagle_game::prelude::Game;
use eagle_server::repository::{Repository, RepositoryLogEntry};

#[derive(Debug, Clone)]
pub(crate) struct GameLog<T: Game> {
    config: T::Config,
    rnd_seed: [u8; 32],
    entries: Vec<RepositoryLogEntry<T>>,
}

impl<T: Game> GameLog<T> {
    pub fn new(config: T::Config, rnd_seed: [u8; 32]) -> Self {
        Self {
            config,
            rnd_seed,
            entries: Vec::new(),
        }
    }
}

impl<T: Game> Repository<T> for GameLog<T> {
    fn store_entry(&mut self, entry: RepositoryLogEntry<T>) {
        self.entries.push(entry);
    }

    fn is_command_handled(
        &self,
        client_id: eagle_types::ids::ClientId,
        index: eagle_types::messages::ClientCommandIndex,
    ) -> bool {
        todo!()
    }
}
