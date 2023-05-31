use crate::ids::PlayerId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameServerInput<ConductorEvent, PlayerEvent> {
    ConductorEvent(ConductorEvent),
    PlayerEvent {
        player_id: PlayerId,
        event: PlayerEvent

    },
    RoomEvents(RoomEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoomEvent {
}
