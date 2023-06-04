use eagle_types::ids::{PlayerId, GameInstanceId};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::Clients,
    event_history::EventHistory,
    game_instances::GameInstances,
    Context, Game, GameHandle, EffHandler,
};

pub struct Room {
    event_history: EventHistory,
    game_instances: GameInstances,
    rng: ChaCha8Rng,
}

impl Room {
    pub fn new<T: Game>(
        root_game_instance_id: GameInstanceId,
        config: T::Config,
        rand_seed: [u8; 32],
    ) -> Self {
        let mut room = Room {
            event_history: EventHistory::new(),
            game_instances: GameInstances::new(),
            rng: ChaCha8Rng::from_seed(rand_seed),
        };
        let handle = GameHandle::new(root_game_instance_id);
        let game = T::new(config);
        room.game_instances.insert_game_instance(handle, game);
        room
    }

    pub fn handle_conductor_event<T: Game>(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        handle: GameHandle<T>,
        event: T::ConductorClientEvent,
    ) {
        let mut ctx = Context::new(
            handle,
            clients,
            eff,
            &mut self.event_history,
            &mut self.game_instances,
            &mut self.rng,
        );
        ctx.handle_conductor_event(handle, event);
    }

    pub fn handle_player_event<T: Game>(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerClientEvent,
    ) {
        let mut ctx = Context::new(
            handle,
            clients,
            eff,
            &mut self.event_history,
            &mut self.game_instances,
            &mut self.rng,
        );
        ctx.handle_player_event(handle, player_id, event);
    }
}
