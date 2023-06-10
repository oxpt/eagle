#![allow(dead_code)]

use eagle_game::prelude::*;

use crate::{
    conductor_model::UltimatumConductor,
    config::UltimatumConfig,
    error::UltimatumError,
    events::{
        UltimatumConductorCommand, UltimatumConductorNotify, UltimatumPlayerCommand,
        UltimatumPlayerNotify,
    },
    phase::Phase,
    player_model::PlayerModel,
    types::{Players, ProposalOpenTiming},
};

pub struct UltimatumGame {
    config: UltimatumConfig,
    phase: Phase,
}

impl Game for UltimatumGame {
    type Config = UltimatumConfig;
    type ConductorNotify = UltimatumConductorNotify;
    type ConductorCommand = UltimatumConductorCommand;
    type PlayerNotify = UltimatumPlayerNotify;
    type PlayerCommand = UltimatumPlayerCommand;
    type Conductor = UltimatumConductor;
    type Player = PlayerModel;

    fn new(config: Self::Config) -> Self {
        Self {
            config,
            phase: Phase::WaitingForAttachment {
                proposer: None,
                responder: None,
            },
        }
    }

    fn name() -> &'static str {
        "Ultimatum"
    }

    fn handle_conductor_command(
        &mut self,
        context: &mut impl GameContext<Self>,
        command: Self::ConductorCommand,
    ) {
        use UltimatumConductorCommand as Command;
        match (&mut self.phase, command) {
            (
                Phase::WaitingForAttachment {
                    responder,
                    proposer,
                },
                Command::AttachProposer(player_id),
            ) => {
                if let Some(responder) = responder {
                    self.phase = Phase::Standby {
                        players: Players {
                            proposer: player_id,
                            responder: *responder,
                        },
                    }
                } else {
                    *proposer = Some(player_id)
                }
            }
            (
                Phase::WaitingForAttachment {
                    proposer,
                    responder,
                },
                Command::AttachResponder(player_id),
            ) => {
                if let Some(proposer) = proposer {
                    self.phase = Phase::Standby {
                        players: Players {
                            proposer: *proposer,
                            responder: player_id,
                        },
                    }
                } else {
                    *responder = Some(player_id)
                }
            }
            (Phase::Standby { players }, Command::StartGame) => {
                self.phase = Phase::Requesting {
                    players: *players,
                    proposal: None,
                }
            }
            (Phase::ProposalHidden { players, proposal }, Command::OpenProposal) => {
                self.phase = Phase::Responding {
                    players: *players,
                    proposal: *proposal,
                }
            }
            (phase, command) => context.push_conductor_notify(UltimatumConductorNotify::Error(
                UltimatumError::UnexpectedConductorCommand {
                    phase: phase.clone(),
                    command,
                },
            )),
        };
    }

    fn handle_player_command(
        &mut self,
        context: &mut impl GameContext<Self>,
        player_id: PlayerId,
        command: Self::PlayerCommand,
    ) {
        use UltimatumPlayerCommand as Command;
        match (&mut self.phase, command) {
            (Phase::Requesting { proposal, .. }, Command::UpdateProposal(new)) => {
                *proposal = Some(new);
            }
            (Phase::Requesting { players, .. }, Command::SubmitProposal(proposal)) => {
                match self.config.proposal_open_timing {
                    ProposalOpenTiming::Immediate => {
                        self.phase = Phase::Responding {
                            players: *players,
                            proposal,
                        }
                    }
                    ProposalOpenTiming::ByConductor => {
                        self.phase = Phase::ProposalHidden {
                            players: *players,
                            proposal,
                        }
                    }
                }
            }
            (Phase::Responding { players, proposal }, Command::Respond(response)) => {
                self.phase = Phase::Result {
                    players: *players,
                    proposal: *proposal,
                    response,
                };
            }
            (phase, command) => context.push_conductor_notify(UltimatumConductorNotify::Error(
                UltimatumError::UnexpectedPlayerCommand {
                    phase: phase.clone(),
                    player_id,
                    command,
                },
            )),
        }
    }

    fn handle_system_command(
        &mut self,
        _context: &mut impl GameContext<Self>,
        _command: SystemCommand,
    ) {
        todo!()
    }
}
