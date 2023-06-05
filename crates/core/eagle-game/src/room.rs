use eagle_types::{
    events::{ClientEventIndex, SystemEvent},
    ids::{GameInstanceId, PlayerId},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::Clients, event_history::EventHistory, game_instances::GameInstances, Context,
    EffHandler, Game, GameHandle,
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

    pub fn handle_system_event<T: Game>(
        &mut self,
        clients: &mut Clients,
        eff: &mut EffHandler,
        handle: GameHandle<T>,
        event: SystemEvent,
    ) {
        let mut ctx = Context::new(
            handle,
            clients,
            eff,
            &mut self.event_history,
            &mut self.game_instances,
            &mut self.rng,
        );
        ctx.handle_system_event(handle, event);
    }

    pub fn log_error<T: Game>(&mut self, handle: GameHandle<T>, error: anyhow::Error) {
        self.game_instances
            .get_game_instance_mut(handle)
            .borrow_mut()
            .log_error(error.into());
    }

    pub fn get_conductor_server_events<T: Game>(
        &mut self,
        handle: GameHandle<T>,
    ) -> impl Iterator<Item = &T::ConductorServerEvent> {
        self.event_history.get_conductor_server_events(handle)
    }

    pub fn get_player_server_events<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &T::PlayerServerEvent> {
        self.event_history
            .get_player_server_events(handle, player_id)
    }

    pub fn current_conductor_client_event_index<T: Game>(
        &self,
        handle: GameHandle<T>,
    ) -> ClientEventIndex {
        self.event_history
            .current_conductor_client_event_index(handle)
    }

    pub fn current_player_client_event_index<T: Game>(
        &self,
        handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> ClientEventIndex {
        self.event_history
            .current_player_client_event_index(handle, player_id)
    }
}
