use eagle_types::ids::{GameInstanceId, PlayerId};

use crate::{Client, Game};

pub trait GameContext<G: Game> {
    // meta informations
    fn count_player_channels(&self, player_id: PlayerId) -> Option<usize>;
    fn count_conductor_channels(&self) -> usize;

    // history
    fn handled_conductor_client_events<T: Game>(
        &self,
        handle: GameHandle<T>,
    ) -> &[<T::Conductor as Client>::ClientEvent];
    fn handled_player_client_events<T: Game>(
        &self,
        handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> &[<T::Player as Client>::ClientEvent];
    fn sent_conductor_server_events<T: Game>(
        &self,
        handle: GameHandle<T>,
    ) -> &[<T::Conductor as Client>::ServerEvent];
    fn sent_player_server_events<T: Game>(
        &self,
        handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> &[<T::Player as Client>::ServerEvent];

    // game output
    fn game_handle() -> GameHandle<G>;
    fn push_server_event_to_conductor<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: <T::Conductor as Client>::ServerEvent,
    );
    fn push_server_event_to_player<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        player_id: PlayerId,
        event: <T::Player as Client>::ServerEvent,
    );

    // game management
    fn create_game_instance<T: Game>(&mut self, config: T::Config) -> GameHandle<T>;
    /// Get the game state for a given game instance.
    fn get_game_state<T: Game>(&self, handle: GameHandle<T>) -> T::State;
    /// Trigger a conductor event for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_conductor_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: <T::Conductor as Client>::ClientEvent,
    );
}

pub struct GameHandle<T: Game> {
    pub game_instance_id: GameInstanceId,
    phantom: std::marker::PhantomData<T>,
}
