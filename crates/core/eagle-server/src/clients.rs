use std::collections::BTreeMap;

use chrono::Utc;
use eagle_game::{clients::ClientsRef, prelude::Game};
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
    pub(crate) fn new(id: ClientId, channel: C) -> Self {
        Self {
            id,
            channel,
            state: ClientState::default(),
        }
    }
    pub fn send_message<T: Serialize>(&mut self, view: ServerToClientMessage<T>) {
        if let Err(err) = self.channel.send_message(view) {
            tracing::error!("failed to notify new view to conductor: {}", err);
            self.state.update_last_error(Utc::now());
        } else {
            self.state.update_last_successful_communication(Utc::now());
        }
    }
}

impl<C: Channel> Clients<C> {
    pub(crate) fn new() -> Self {
        Self {
            users: Default::default(),
        }
    }

    pub(crate) fn add_client(&mut self, user: User, client: Client<C>) {
        self.users
            .entry(user)
            .or_insert_with(BTreeMap::new)
            .insert(client.id, client);
    }

    pub(crate) fn remove_client(&mut self, user: User, client_id: ClientId) {
        if let Some(clients) = self.users.get_mut(&user) {
            clients.remove(&client_id);
        }
    }

    pub(crate) fn notify_to_conductor<T: Game>(&mut self, view: &T::ConductorView) {
        for client in self
            .users
            .get_mut(&User::Conductor)
            .into_iter()
            .flat_map(|clients| clients.values_mut())
        {
            client.send_message(ServerToClientMessage::Notify { view: view.clone() });
        }
    }

    pub(crate) fn notify_to_player<T: Game>(&mut self, player_id: PlayerId, view: &T::PlayerView) {
        for client in self
            .users
            .get_mut(&User::Player(player_id))
            .into_iter()
            .flat_map(|clients| clients.values_mut())
        {
            client.send_message(ServerToClientMessage::Notify { view: view.clone() });
        }
    }

    pub(crate) fn get_client_mut(
        &mut self,
        user: User,
        client_id: ClientId,
    ) -> Option<&mut Client<C>> {
        self.users
            .get_mut(&user)
            .and_then(|clients| clients.get_mut(&client_id))
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

    pub(crate) fn available_players(&self) -> impl Iterator<Item = PlayerId> + '_ {
        self.users.iter().filter_map(|(user, clients)| match user {
            User::Player(player_id) => {
                if clients.is_empty() {
                    // No need to notify to the player who is not connected.
                    None
                } else {
                    Some(*player_id)
                }
            }
            _ => None,
        })
    }
}
