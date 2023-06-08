use std::{any::Any, collections::BTreeMap};

use crate::{events::GameCommand, game::handle::GameHandle, game::Game};
use eagle_types::{
    events::SystemCommand,
    ids::{GameInstanceId, PlayerId},
};

#[derive(Default)]
/// This stores server and client events in RON format.
pub(crate) struct CommandHistory {
    /// Used to provide a history method for public API.
    // Any is CommandLog<T> where T: Game
    games: BTreeMap<GameInstanceId, Box<dyn Any>>,
}

struct CommandLog<T: Game> {
    commands: Vec<GameCommand<T>>,
}

impl<T: Game> CommandLog<T> {
    fn new() -> Self {
        Self {
            commands: Default::default(),
        }
    }
}

impl CommandHistory {
    pub fn new() -> Self {
        Default::default()
    }

    fn push_command<T: Game>(&mut self, game_handle: GameHandle<T>, command: GameCommand<T>) {
        self.games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(CommandLog::<T>::new()))
            .downcast_mut::<CommandLog<T>>()
            .unwrap()
            .commands
            .push(command)
    }

    pub fn log_conductor_command<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        command: T::ConductorCommand,
    ) {
        self.push_command(game_handle, GameCommand::ConductorCommand(command));
    }

    pub fn log_player_command<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
        command: T::PlayerCommand,
    ) {
        self.push_command(game_handle, GameCommand::PlayerCommand(player_id, command));
    }

    pub fn log_system_command<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        command: SystemCommand,
    ) {
        self.push_command(game_handle, GameCommand::SystemCommand(command));
    }

    pub fn all_commands<T: Game>(
        &self,
        game_handle: GameHandle<T>,
    ) -> std::slice::Iter<'_, GameCommand<T>> {
        self.games
            .get(&game_handle.game_instance_id)
            .map(|commands| {
                commands
                    .downcast_ref::<CommandLog<T>>()
                    .unwrap()
                    .commands
                    .iter()
            })
            .unwrap_or_default()
    }
}
