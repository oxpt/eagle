use std::any::Any;

use eagle_types::{
    client::{ClientState, User},
    ids::GameInstanceId,
};
use serde::Serialize;

#[derive(Clone, Copy)]
pub struct Clients<'a> {
    pub inner: &'a dyn Any,
    pub fn_get_client_states: fn(&'a dyn Any, User) -> Vec<ClientState>,
    pub fn_send_server_event:
        fn(&'a dyn Any, User, GameInstanceId, usize, &dyn erased_serde::Serialize),
}

impl Clients<'_> {
    pub(crate) fn get_client_states(&self, user: User) -> Vec<ClientState> {
        (self.fn_get_client_states)(self.inner, user)
    }
    pub(crate) fn send_server_event<T: Serialize>(
        &self,
        user: User,
        game_instance_id: GameInstanceId,
        index: usize,
        event: T,
    ) {
        (self.fn_send_server_event)(self.inner, user, game_instance_id, index, &event);
    }
}
