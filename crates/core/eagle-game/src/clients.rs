use std::any::Any;

use eagle_types::{
    client::{ClientState, User},
    events::ServerEventIndex,
    ids::GameInstanceId,
};
use serde::Serialize;

pub struct Clients<'a> {
    pub inner: &'a mut dyn Any,
    pub fn_get_client_states: fn(&dyn Any, User) -> Vec<ClientState>,
    pub fn_send_server_event:
        fn(&mut dyn Any, User, GameInstanceId, ServerEventIndex, &dyn erased_serde::Serialize),
}

impl Clients<'_> {
    pub(crate) fn get_client_states(&self, user: User) -> Vec<ClientState> {
        (self.fn_get_client_states)(self.inner, user)
    }
    pub(crate) fn send_server_event<T: Serialize>(
        &mut self,
        user: User,
        game_instance_id: GameInstanceId,
        index: ServerEventIndex,
        event: T,
    ) {
        (self.fn_send_server_event)(self.inner, user, game_instance_id, index, &event);
    }
}
