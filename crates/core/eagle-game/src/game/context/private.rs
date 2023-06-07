use eagle_types::{ids::PlayerId, events::SystemCommand};

use crate::{game::Game, game::handle::GameHandle};

use super::GameContext;

impl<'a, 'client, T: Game> GameContext<'a, 'client, T> {
    pub(crate) fn handle_conductor_command(
        &mut self,
        handle: GameHandle<T>,
        game: &mut T,
        command: T::ConductorCommand,
    ) {
        self.command_history
            .log_conductor_command(handle, command.clone());
        game.handle_conductor_command(self, command);
    }

    pub(crate) fn handle_player_command(
        &mut self,
        handle: GameHandle<T>,
        game: &mut T,
        player_id: PlayerId,
        command: T::PlayerCommand,
    ) {
        self.command_history
            .log_player_command(handle, player_id, command.clone());
        game.handle_player_command(self, player_id, command);
    }

    pub(crate) fn handle_system_command(
        &mut self,
        handle: GameHandle<T>,
        game: &mut T,
        command: SystemCommand,
    ) {
        self.command_history
            .log_system_command(handle, command.clone());
        game.handle_system_command(self, command);
    }
}
