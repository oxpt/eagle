use std::cell::RefCell;

use eagle_types::{
    events::{SystemCommand},
    ids::{GameInstanceId, PlayerId},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::Clients, command_history::CommandHistory, game_instances::GameInstances,
    notify_history::NotifyHistory, Context, EffHandler, Game, GameHandle,
};

pub struct Room<T: Game> {
    game_handle: GameHandle<T>,
    command_history: CommandHistory,
    notify_history: NotifyHistory<T>,
    game_instances: GameInstances,
    rng: ChaCha8Rng,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Game> Room<T> {
    pub fn new(
        root_game_instance_id: GameInstanceId,
        config: T::Config,
        rand_seed: [u8; 32],
    ) -> Self {
        let mut room = Room {
            game_handle: GameHandle::new(root_game_instance_id),
            command_history: CommandHistory::new(),
            notify_history: NotifyHistory::new(),
            game_instances: GameInstances::new(),
            rng: ChaCha8Rng::from_seed(rand_seed),
            _phantom: std::marker::PhantomData,
        };
        let handle = GameHandle::new(root_game_instance_id);
        let game = T::new(config);
        room.game_instances.insert_game_instance(handle, game);
        room
    }

    fn mutate_game(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        mutate: impl FnOnce(&mut Context<T>, &RefCell<T>),
    ) {
        let game = &self.game_instances.get_game_instance_mut(self.game_handle);

        let mut ctx = Context::new(
            self.game_handle,
            clients,
            eff,
            &mut self.command_history,
            &mut self.notify_history,
            &mut self.game_instances,
            &mut self.rng,
        );

        mutate(&mut ctx, game);
    }

    pub fn handle_conductor_command(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        command: T::ConductorCommand,
    ) {
        self.command_history.log_conductor_command(self.game_handle, command.clone());
        self.mutate_game(clients, eff, |ctx, game| {
            game.borrow_mut().handle_conductor_command(ctx, command);
        });
    }

    pub fn handle_player_command(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        player_id: PlayerId,
        command: T::PlayerCommand,
    ) {
        self.command_history
            .log_player_command(self.game_handle, player_id, command.clone());
        self.mutate_game(clients, eff, |ctx, game| {
            game.borrow_mut()
                .handle_player_command(ctx, player_id, command);
        });
    }

    pub fn handle_system_command(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        event: SystemCommand,
    ) {
        self.command_history
            .log_system_command(self.game_handle, event.clone());
        self.mutate_game(clients, eff, |ctx, game| {
            game.borrow_mut().handle_system_command(ctx, event);
        });
    }

    pub fn get_conductor_notifies(&mut self) -> impl Iterator<Item = &T::ConductorNotify> {
        self.notify_history.get_conductor_notifies()
    }

    pub fn get_player_notifies(
        &mut self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &T::PlayerNotify> {
        self.notify_history.get_player_notifies(player_id)
    }
}
