use eagle_types::{
    client::User,
    ids::{ClientId, PlayerId},
};
use serde::Deserializer;

use crate::{channel::Channel, channels::Channels, room_state::RoomState};

pub struct GameServer<C: Channel> {
    channels: Channels<C>,
    room_state: RoomState,
}

impl<T: Channel> GameServer<T> {
    fn add_conductor_channel(&mut self, channel: T) {
        self.channels.add_channel(channel.client_id(), channel);
        todo!()
    }
    fn add_player_channel(&mut self, player_id: PlayerId, channel: T) {
        self.channels.add_channel(channel.client_id(), channel);
        todo!()
    }

    fn handle_message(&mut self, channel_type: User, msg: impl Deserializer<'static>) {
        match channel_type {
            User::Conductor => {}
            User::Player(player_id) => {}
        }
    }

    fn get_client_type(&mut self, client_id: ClientId) -> Option<User> {
        todo!()
    }
}
