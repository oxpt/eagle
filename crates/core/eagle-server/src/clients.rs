use std::collections::BTreeMap;

use chrono::Utc;
use eagle_game::clients::ClientsRef;
use eagle_types::{
    client::{ClientState, User},
    ids::ClientId,
};

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

    pub(crate) fn add_client(&mut self, user: User, client_id: ClientId, channel: C) {
        self.users.entry(user).or_insert_with(BTreeMap::new).insert(
            client_id,
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

    pub(crate) fn clients_ref(&mut self) -> ClientsRef<'_> {
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
