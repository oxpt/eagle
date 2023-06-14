use std::any::Any;

use eagle_types::client::{ClientState, User};

pub struct ClientsRef<'a> {
    pub inner: &'a dyn Any,
    pub fn_get_client_states: fn(&dyn Any, User) -> Vec<ClientState>,
}

impl ClientsRef<'_> {
    pub(crate) fn get_client_states(&self, user: User) -> Vec<ClientState> {
        (self.fn_get_client_states)(self.inner, user)
    }
}
