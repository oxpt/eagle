use eagle_game::{
    eff_handler::EffHandler,
    prelude::{Game, GameCommand},
    room::Room,
};
use eagle_types::{
    client::User,
    events::SystemCommand,
    ids::{ClientId, PlayerId},
};

use crate::{
    channel::Channel,
    clients::Clients,
    effect_outcomes::EffectOutcomes,
    repository::{Repository, RepositoryLogEntry},
};

pub struct GameServer<T: Game, C: Channel> {
    clients: Clients<C>,
    room: Room<T>,
}

impl<T: Game, C: Channel> GameServer<T, C> {
    pub fn new(room: Room<T>) -> Self {
        Self {
            clients: Clients::new(),
            room,
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
        self.clients.remove_channel(user, client_id);
    }

    pub fn handle_conductor_command(
        &mut self,
        repository: &mut impl Repository<T>,
        client_id: ClientId,
        command: T::ConductorCommand,
    ) {
        // TODO: skip already received events
        let mut eff = EffHandler::default();
        self.room.handle_conductor_command(
            &mut self.clients.clients_ref(),
            &mut eff,
            command.clone(),
        );
        let effect_outcomes = EffectOutcomes::from(eff);
        self.clients
            .update_last_successful_communication(User::Conductor, client_id);
        repository.store_command(RepositoryLogEntry {
            command: GameCommand::ConductorCommand(command),
            effect_outcomes,
        });
    }

    pub fn handle_player_command(
        &mut self,
        repository: &mut impl Repository<T>,
        client_id: ClientId,
        player_id: PlayerId,
        command: T::PlayerCommand,
    ) {
        // TODO: skip already received events
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
        repository.store_command(RepositoryLogEntry {
            command: GameCommand::PlayerCommand(player_id, command),
            effect_outcomes,
        });
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
        repository.store_command(RepositoryLogEntry {
            command: GameCommand::SystemCommand(command),
            effect_outcomes,
        });
    }

    pub fn replay_conductor_event(
        &mut self,
        event: T::ConductorCommand,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_conductor_command(&mut self.clients.clients_ref(), &mut eff, event);
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
    }

    pub fn replay_system_event(&mut self, event: SystemCommand, effect_outcomes: EffectOutcomes) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_system_command(&mut self.clients.clients_ref(), &mut eff, event);
    }
}
