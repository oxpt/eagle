use eagle_types::{
    events::SystemCommand,
    ids::{GameInstanceId, PlayerId},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::ClientsRef, command_history::CommandHistory, context::Context, eff_handler::EffHandler,
    game::Game, game_handle::GameHandle, game_instances::GameInstances,
    notify_history::NotifyHistory,
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
        clients: &mut ClientsRef,
        eff: &mut EffHandler,
        mutate: impl FnOnce(&mut Context<T>, &mut T),
    ) {
        let game = self.game_instances.get_game_instance_mut(self.game_handle);

        let mut ctx = Context::new(
            self.game_handle,
            clients,
            eff,
            &mut self.command_history,
            &mut self.notify_history,
            &mut self.game_instances,
            &mut self.rng,
        );

        mutate(&mut ctx, &mut game.borrow_mut());
    }

    pub fn handle_conductor_command(
        &mut self,
        clients: &mut ClientsRef,
        eff: &mut EffHandler,
        command: T::ConductorCommand,
    ) {
        let handle = self.game_handle;
        self.mutate_game(clients, eff, |ctx, game| {
            ctx.handle_conductor_command(handle, game, command);
        });
    }

    pub fn handle_player_command(
        &mut self,
        clients: &mut ClientsRef,
        eff: &mut EffHandler,
        player_id: PlayerId,
        command: T::PlayerCommand,
    ) {
        let handle = self.game_handle;
        self.mutate_game(clients, eff, |ctx, game| {
            ctx.handle_player_command(handle, game, player_id, command);
        });
    }

    pub fn handle_system_command(
        &mut self,
        clients: &mut ClientsRef,
        eff: &mut EffHandler,
        command: SystemCommand,
    ) {
        let handle = self.game_handle;
        self.mutate_game(clients, eff, |ctx, game| {
            ctx.handle_system_command(handle, game, command);
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
