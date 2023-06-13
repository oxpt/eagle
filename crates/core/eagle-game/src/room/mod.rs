pub(crate) mod command_history;
pub(crate) mod game_instances;

use eagle_types::{
    events::SystemCommand,
    ids::{GameInstanceId, PlayerId},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use self::{command_history::CommandHistory, game_instances::GameInstances};
use crate::{
    clients::ClientsRef,
    eff_handler::EffHandler,
    game::{context::GameContextImpl, handle::GameHandle, Game},
};

pub struct Room<T: Game> {
    game_handle: GameHandle<T>,
    command_history: CommandHistory,
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
        mutate: impl FnOnce(&mut GameContextImpl<T>, &mut T),
    ) {
        let game = self.game_instances.get_game_instance(self.game_handle);

        let mut ctx = GameContextImpl::new(
            self.game_handle,
            clients,
            eff,
            &mut self.command_history,
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
}
