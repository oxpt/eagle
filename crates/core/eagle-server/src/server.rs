use eagle_game::{prelude::{Game, GameCommand, GameHandle}, room::Room, eff_handler::EffHandler};
use eagle_types::{
    client::{ClientParams, User},
    events::{CommandIndex, SystemCommand},
    ids::{ClientId, PlayerId},
};

use crate::{notify_sender::NotifySender, clients::Clients, repository::Repository, EffectOutcomes};

pub struct GameServer<T: Game, C: NotifySender> {
    clients: Clients<C>,
    room: Room<T>,
}

impl<T: Game, C: NotifySender> GameServer<T, C> {
    pub fn new(room: Room<T>) -> Self {
        Self {
            clients: Clients::new(),
            room,
        }
    }

    pub fn add_conductor_client(&mut self, params: ClientParams, channel: C) {
        self.clients.add_client(User::Conductor, channel);
        if let Some(mut latest_index) = params.latest_received_server_event {
            let events = self.room.get_conductor_notifies();
            for event in events.skip(latest_index.skip()) {
                latest_index = latest_index.next();
                self.clients
                    .send_server_event(User::Conductor, latest_index, event);
            }
        }
    }

    pub fn add_player_client(&mut self, player_id: PlayerId, params: ClientParams, channel: C) {
        self.clients.add_client(User::Player(player_id), channel);
        if let Some(mut latest_index) = params.latest_received_server_event {
            let events = self.room.get_player_notifies(player_id);
            for event in events.skip(latest_index.skip()) {
                latest_index = latest_index.next();
                self.clients
                    .send_server_event(User::Player(player_id), latest_index, event);
            }
        }
    }

    pub fn remove_client(&mut self, user: User, client_id: ClientId) {
        self.clients.remove_channel(user, client_id);
    }

    pub fn handle_conductor_event(
        &mut self,
        repository: &mut impl Repository<T>,
        client_id: ClientId,
        index: CommandIndex,
        command: T::ConductorCommand,
    ) {
        // TODO: skip already received events
        let mut eff = EffHandler::default();
        self.room
            .handle_conductor_command(&mut self.clients.to_ref(), &mut eff, command.clone());
        let effect_outcomes = EffectOutcomes::from(eff);
        self.clients
            .update_last_successful_communication(User::Conductor, client_id);
        repository.store_command(GameCommand::ConductorCommand(command), effect_outcomes);
    }

    pub fn handle_player_event(
        &mut self,
        repository: &mut impl Repository<T>,
        client_id: ClientId,
        player_id: PlayerId,
        index: CommandIndex,
        command: T::PlayerCommand,
    ) {
        // TODO: skip already received events
        let mut eff = EffHandler::default();
        self.room.handle_player_command(
            &mut self.clients.to_ref(),
            &mut eff,
            player_id,
            command.clone(),
        );
        let effect_outcomes = EffectOutcomes::from(eff);
        self.clients
            .update_last_successful_communication(User::Player(player_id), client_id);
        repository.store_command(GameCommand::PlayerCommand(player_id, command), effect_outcomes);
    }

    pub fn handle_system_event(
        &mut self,
        repository: &mut impl Repository<T>,
        command: SystemCommand,
    ) {
        let mut eff = EffHandler::default();
        self.room
            .handle_system_command(&mut self.clients.to_ref(), &mut eff, command.clone());
        let effect_outcomes = EffectOutcomes::from(eff);
        repository.store_command(GameCommand::SystemCommand(command), effect_outcomes);
    }

    pub fn replay_conductor_event(
        &mut self,
        event: T::ConductorCommand,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_conductor_command(&mut self.clients.to_ref(), &mut eff, event);
    }

    pub fn replay_player_event(
        &mut self,
        player_id: PlayerId,
        event: T::PlayerCommand,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_player_command(&mut self.clients.to_ref(), &mut eff, player_id, event);
    }

    pub fn replay_system_event(&mut self, event: SystemCommand, effect_outcomes: EffectOutcomes) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_system_command(&mut self.clients.to_ref(), &mut eff, event);
    }
}
