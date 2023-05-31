use eagle_types::{ids::PlayerId, events::GameServerInput};

pub trait SystemState {
    type PlayerServerEvent;
    type ConductorServerEvent;
    fn count_player_channels(&self, player_id: PlayerId) -> Option<usize>;
    fn count_conductor_channels(&self) -> usize;
    fn handled_server_events(&self) -> &[GameServerInput<Self::ConductorServerEvent, Self::PlayerServerEvent>];
    fn yielded_player_events(&self, player_id: PlayerId) -> &[Self::PlayerServerEvent];
    fn yielded_conductor_events(&self) -> &[Self::ConductorServerEvent];
}
