use std::collections::BTreeMap;

use chrono::Utc;
use eagle_game::clients::ClientsRef;
use eagle_types::{
    client::{ClientState, User},
    events::NotifyIndex,
    ids::ClientId,
};
use serde::Serialize;

use crate::channel::Channel;

pub struct Clients<C: Channel> {
    users: BTreeMap<User, BTreeMap<ClientId, Client<C>>>,
}

struct Client<C: Channel> {
    channel: C,
    state: ClientState,
}

impl<C: Channel> Clients<C> {
    pub(crate) fn new() -> Self {
        Self {
            users: Default::default(),
        }
    }

    pub(crate) fn add_client(&mut self, user: User, channel: C) {
        self.users.entry(user).or_insert_with(BTreeMap::new).insert(
            channel.client_id(),
            Client {
                channel,
                state: ClientState::default(),
            },
        );
    }

    pub(crate) fn remove_channel(&mut self, user: User, client_id: ClientId) {
        if let Some(clients) = self.users.get_mut(&user) {
            clients.remove(&client_id);
        }
    }

    pub(crate) fn send_server_event<T: Clone + Serialize>(
        &mut self,
        user: User,
        index: NotifyIndex,
        event: T,
    ) {
        if let Some(clients) = self.users.get_mut(&user) {
            let mut failures = Vec::new();
            for (_, client) in clients.iter_mut() {
                // TODO: Don't send if the client already receives the event.
                if let Err(_err) = client.channel.send(index, event.clone()) {
                    failures.push(client.channel.client_id());
                }
            }
        }
    }

    pub(crate) fn to_ref(&mut self) -> ClientsRef<'_> {
        ClientsRef {
            inner: self,
            fn_get_client_states: |inner, user| {
                inner
                    .downcast_ref::<Clients<C>>()
                    .unwrap()
                    .users
                    .get(&user)
                    .map(|clients| clients.iter().map(|(_, client)| client.state).collect())
                    .unwrap_or_default()
            },
            fn_send_server_event: |inner, user, index, event| {
                inner
                    .downcast_mut::<Clients<C>>()
                    .unwrap()
                    .send_server_event(user, index, event)
            },
        }
    }

    pub(crate) fn update_last_successful_communication(&mut self, user: User, client_id: ClientId) {
        if let Some(user) = self.users.get_mut(&user) {
            if let Some(client) = user.get_mut(&client_id) {
                client
                    .state
                    .update_last_successful_communication(Utc::now());
            }
        }
    }
}
