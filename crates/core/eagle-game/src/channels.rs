use std::{any::Any, collections::BTreeMap};

use eagle_types::{client::ClientState, ids::ClientId, event::IndexedEvent};

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

    pub(crate) fn send_server_event(&mut self, client_id: ClientId, event: IndexedEvent) {
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

    pub(crate) fn to_ref(&self) -> ChannelsRef<'_> {
        ChannelsRef {
            inner: self,
            get_client_state: |inner, client_id| {
                let channels = inner.downcast_ref::<Channels<C>>().unwrap();
                channels
                    .get_channel(client_id)
                    .map(|channel| channel.client_state())
            },
            send_server_event: |inner, client_id, event| {
                let channels = inner.downcast_ref::<Channels<C>>().unwrap();
                channels.send_server_event(client_id, event);
            }
        }
    }
}

pub(crate) struct ChannelsRef<'a> {
    inner: &'a dyn Any,
    get_client_state: fn(&'a dyn Any, ClientId) -> Option<ClientState>,
    send_server_event: fn(&'a dyn Any, ClientId, IndexedEvent),
}

impl ChannelsRef<'_> {
    pub(crate) fn get_client_state(&self, client_id: ClientId) -> Option<ClientState> {
        (self.get_client_state)(self.inner, client_id)
    }
    pub(crate) fn send_server_event(&self, client_id: ClientId, event: IndexedEvent) {
        (self.send_server_event)(self.inner, client_id, event);
    }
}
