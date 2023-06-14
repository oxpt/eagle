use std::collections::BTreeMap;

use chrono::Utc;
use eagle_game::clients::ClientsRef;
use eagle_types::{
    client::{ClientState, User},
    ids::{ClientId, PlayerId},
    messages::ServerToClientMessage,
};
use serde::Serialize;

use crate::channel::Channel;

pub struct Clients<C: Channel> {
    users: BTreeMap<User, BTreeMap<ClientId, Client<C>>>,
}

pub(crate) struct Client<C: Channel> {
    pub id: ClientId,
    channel: C,
    state: ClientState,
}

impl<C: Channel> Client<C> {
    pub fn send_message<T: Serialize>(
        &self,
        view: ServerToClientMessage<T>,
    ) -> Result<(), C::Error> {
        self.channel.send_message(view)
    }
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
                id: client_id,
                channel,
                state: ClientState::default(),
            },
        );
    }

    pub(crate) fn remove_client(&mut self, user: User, client_id: ClientId) {
        if let Some(clients) = self.users.get_mut(&user) {
            clients.remove(&client_id);
        }
    }

    pub(crate) fn get_client(&self, user: User, client_id: ClientId) -> Option<&Client<C>> {
        self.users
            .get(&user)
            .and_then(|clients| clients.get(&client_id))
    }

    pub(crate) fn clients_ref(&self) -> ClientsRef<'_> {
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

    pub(crate) fn conductor_clients(&self) -> impl Iterator<Item = &Client<C>> {
        self.users
            .get(&User::Conductor)
            .into_iter()
            .flat_map(|clients| clients.values())
    }

    pub(crate) fn players(
        &self,
    ) -> impl Iterator<Item = (PlayerId, impl Iterator<Item = &Client<C>>)> {
        self.users.iter().filter_map(|(user, clients)| match user {
            User::Player(player_id) => Some((*player_id, clients.values())),
            _ => None,
        })
    }
}
