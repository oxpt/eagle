use std::any::Any;

use eagle_types::{
    client::{ClientState, User},
    events::NotifyIndex,
};
use serde::Serialize;

pub struct ClientsRef<'a> {
    pub inner: &'a mut dyn Any,
    pub fn_get_client_states: fn(&dyn Any, User) -> Vec<ClientState>,
    pub fn_send_server_event: fn(&mut dyn Any, User, NotifyIndex, &dyn erased_serde::Serialize),
}

impl ClientsRef<'_> {
    pub(crate) fn get_client_states(&self, user: User) -> Vec<ClientState> {
        (self.fn_get_client_states)(self.inner, user)
    }
    pub(crate) fn send_notify<T: Serialize>(&mut self, user: User, index: NotifyIndex, event: T) {
        (self.fn_send_server_event)(self.inner, user, index, &event);
    }
}
