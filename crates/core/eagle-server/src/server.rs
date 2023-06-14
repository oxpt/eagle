use eagle_game::{eff_handler::EffHandler, prelude::Game, room::Room};
use eagle_types::{
    client::User,
    events::SystemCommand,
    ids::{ClientId, PlayerId},
    messages::{ClientToServerMessage, ServerToClientMessage},
};

use crate::{
    channel::Channel,
    clients::Clients,
    effect_outcomes::EffectOutcomes,
    last_view::{LastViews, UpdateResult},
    repository::{CommandLogEntry, Repository, RepositoryLogEntry},
};

pub struct GameServer<T: Game, C: Channel> {
    clients: Clients<C>,
    room: Room<T>,
    last_views: LastViews<T>,
}

impl<T: Game, C: Channel> GameServer<T, C> {
    pub fn new(room: Room<T>) -> Self {
        Self {
            clients: Clients::new(),
            room,
            last_views: Default::default(),
        }
    }

    pub fn add_conductor_client(&mut self, client_id: ClientId, channel: C) {
        self.clients.add_client(User::Conductor, client_id, channel);
    }

    pub fn add_player_client(&mut self, player_id: PlayerId, client_id: ClientId, channel: C) {
        self.clients
            .add_client(User::Player(player_id), client_id, channel);
    }

    pub fn remove_client(&mut self, user: User, client_id: ClientId) {
        self.clients.remove_client(user, client_id);
    }

    fn notify_view(&mut self) {
        let conductor_view = self.room.render_conductor();
        if self.last_views.update_conductor_view(&conductor_view) == UpdateResult::Updated {
            for client in self.clients.conductor_clients() {
                if let Err(_err) = client.send_message(ServerToClientMessage::Notify {
                    view: conductor_view.clone(),
                }) {
                    // TODO: log err
                }
            }
        }
        for (player_id, clients) in self.clients.players() {
            let player_view = self.room.render_player(player_id);
            if self.last_views.update_player_view(player_id, &player_view) == UpdateResult::Updated
            {
                for client in clients {
                    if let Err(_err) = client.send_message(ServerToClientMessage::Notify {
                        view: player_view.clone(),
                    }) {
                        // TODO: log err
                    }
                }
            }
        }
    }

    pub fn handle_conductor_command(
        &mut self,
        repository: &mut impl Repository<T>,
        client_id: ClientId,
        message: ClientToServerMessage<T::ConductorCommand>,
    ) {
        match message {
            ClientToServerMessage::Command { index, command } => {
                if repository.is_command_handled(client_id, index) {
                    return;
                }
                let mut eff = EffHandler::default();
                self.room.handle_conductor_command(
                    &mut self.clients.clients_ref(),
                    &mut eff,
                    command.clone(),
                );
                let effect_outcomes = EffectOutcomes::from(eff);
                self.clients
                    .update_last_successful_communication(User::Conductor, client_id);
                repository.store_entry(RepositoryLogEntry::ConductorCommand {
                    index,
                    entry: CommandLogEntry {
                        command,
                        effect_outcomes,
                    },
                });
                if let Some(client) = self.clients.get_client(User::Conductor, client_id) {
                    if let Err(_err) = client
                        .send_message::<T::ConductorCommand>(ServerToClientMessage::Ack { index })
                    {
                        // TODO: log err
                    }
                }
                self.notify_view()
            }
            _ => todo!(),
        };
    }

    pub fn handle_player_command(
        &mut self,
        repository: &mut impl Repository<T>,
        client_id: ClientId,
        player_id: PlayerId,
        message: ClientToServerMessage<T::PlayerCommand>,
    ) {
        match message {
            ClientToServerMessage::Command { index, command } => {
                if repository.is_command_handled(client_id, index) {
                    return;
                }
                let mut eff = EffHandler::default();
                self.room.handle_player_command(
                    &mut self.clients.clients_ref(),
                    &mut eff,
                    player_id,
                    command.clone(),
                );
                let effect_outcomes = EffectOutcomes::from(eff);
                self.clients
                    .update_last_successful_communication(User::Player(player_id), client_id);
                repository.store_entry(RepositoryLogEntry::PlayerCommand {
                    player_id,
                    index,
                    entry: CommandLogEntry {
                        command,
                        effect_outcomes,
                    },
                });
                if let Some(client) = self.clients.get_client(User::Player(player_id), client_id) {
                    if let Err(_err) = client
                        .send_message::<T::PlayerCommand>(ServerToClientMessage::Ack { index })
                    {
                        // TODO: log err
                    }
                }
                self.notify_view()
            }
            _ => todo!(),
        }
    }

    pub fn handle_system_event(
        &mut self,
        repository: &mut impl Repository<T>,
        command: SystemCommand,
    ) {
        let mut eff = EffHandler::default();
        self.room
            .handle_system_command(&mut self.clients.clients_ref(), &mut eff, command.clone());
        let effect_outcomes = EffectOutcomes::from(eff);
        repository.store_entry(RepositoryLogEntry::SystemCommand(CommandLogEntry {
            command,
            effect_outcomes,
        }));
        self.notify_view()
    }

    pub fn replay_conductor_event(
        &mut self,
        event: T::ConductorCommand,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_conductor_command(&mut self.clients.clients_ref(), &mut eff, event);
        // no notify
    }

    pub fn replay_player_event(
        &mut self,
        player_id: PlayerId,
        event: T::PlayerCommand,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room.handle_player_command(
            &mut self.clients.clients_ref(),
            &mut eff,
            player_id,
            event,
        );
        // no notify
    }

    pub fn replay_system_event(&mut self, event: SystemCommand, effect_outcomes: EffectOutcomes) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_system_command(&mut self.clients.clients_ref(), &mut eff, event);
        // no notify
    }
}
