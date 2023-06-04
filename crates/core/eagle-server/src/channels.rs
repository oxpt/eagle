use std::collections::BTreeMap;

use eagle_game::clients::Clients;
use eagle_types::ids::ClientId;
use serde::Serialize;

use crate::channel::Channel;

pub struct Channels<C: Channel> {
    channels: BTreeMap<ClientId, C>,
}

impl<C: Channel> Channels<C> {
    pub(crate) fn new() -> Self {
        Self {
            channels: Default::default(),
        }
    }

    pub(crate) fn add_channel(&mut self, client_id: ClientId, channel: C) {
        self.channels.insert(client_id, channel);
    }

    pub(crate) fn remove_channel(&mut self, client_id: ClientId) {
        self.channels.remove(&client_id);
    }

    pub(crate) fn send_server_event<T: Serialize>(&mut self, client_id: ClientId, event: T) {
        if let Some(channel) = self.channels.get_mut(&client_id) {
            channel.send(event);
        }
    }

    pub(crate) fn close(&mut self, client_id: ClientId) {
        if let Some(channel) = self.channels.get_mut(&client_id) {
            channel.close();
        }
    }

    pub(crate) fn get_channel(&self, client_id: ClientId) -> Option<&C> {
        self.channels.get(&client_id)
    }

    pub(crate) fn to_ref(&self) -> Clients<'_> {
        Clients {
            inner: self,
            fn_get_client_states: |inner, user| {
                todo!()
            },
            fn_send_server_event: |inner, game_instance_id, user, index, event| {
                todo!()
            },
        }
    }
}
