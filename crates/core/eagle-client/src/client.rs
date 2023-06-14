use std::marker::PhantomData;

use eagle_types::messages::ClientToServerMessage;
use serde::Serialize;

use crate::{channel::Channel, repository::Repository};

#[derive(Default)]
pub struct Client<T, C: Channel> {
    command: PhantomData<T>,
    channel: C,
}

impl<T: Clone + Serialize + 'static, C: Channel> Client<T, C> {
    pub fn new(channel: C) -> Self {
        Self {
            command: PhantomData,
            channel,
        }
    }

    pub fn init(&self, repository: &impl Repository<T>) -> Result<(), C::Error> {
        for (index, command) in repository.pending_commands() {
            self.channel
                .send_message(ClientToServerMessage::Command { index, command })?;
        }
        Ok(())
    }

    pub fn send_command(
        &self,
        repository: &mut impl Repository<T>,
        command: T,
    ) -> Result<(), C::Error> {
        let index = repository.push_command(command.clone());
        self.channel
            .send_message(ClientToServerMessage::Command { index, command })
    }
}
