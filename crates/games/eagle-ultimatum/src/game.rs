#![allow(dead_code)]

use eagle_game::prelude::*;

use crate::{
    conductor_model::UltimatumConductor,
    config::UltimatumConfig,
    events::{UltimatumConductorCommand, UltimatumPlayerCommand},
    phase::Phase,
    player_model::UltimatumPlayer,
    types::{Players, Proposal, ProposalOpenTiming},
};

#[derive(Debug)]
pub struct UltimatumGame {
    config: UltimatumConfig,
    phase: Phase,
    conductor: UltimatumConductor,
    players: Map<PlayerId, UltimatumPlayer>,
}

pub(crate) fn standby(context: &mut impl GameContext<UltimatumGame>, players: Players) -> Phase {
    // context.push_player_notify(players.proposer, UltimatumPlayerNotify::YouAreProposer);
    // context.push_player_notify(players.responder, UltimatumPlayerNotify::YouAreResponder);
    return Phase::Standby { players };
}

pub(crate) fn open_proposal(
    context: &mut impl GameContext<UltimatumGame>,
    players: Players,
    proposal: Proposal,
) -> Phase {
    // context.push_player_notify(
    //     players.proposer,
    //     UltimatumPlayerNotify::OpenProposal(proposal),
    // );
    // context.push_player_notify(
    //     players.responder,
    //     UltimatumPlayerNotify::OpenProposal(proposal),
    // );
    return Phase::Responding { players, proposal };
}

impl Game for UltimatumGame {
    type Config = UltimatumConfig;
    type ConductorCommand = UltimatumConductorCommand;
    type PlayerCommand = UltimatumPlayerCommand;
    type ConductorView = UltimatumConductor;
    type PlayerView = UltimatumPlayer;

    fn new(config: Self::Config) -> Self {
        Self {
            config,
            phase: Phase::WaitingForAttachment {
                proposer: None,
                responder: None,
            },
            conductor: UltimatumConductor::new(),
            players: Map::new(),
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
            (Phase::WaitingForAttachment { responder, .. }, Command::AttachProposer(proposer)) => {
                if let Some(responder) = responder {
                    self.phase = standby(
                        context,
                        Players {
                            proposer,
                            responder: *responder,
                        },
                    )
                } else {
                    self.phase = Phase::WaitingForAttachment {
                        proposer: Some(proposer),
                        responder: *responder,
                    }
                }
            }
            (Phase::WaitingForAttachment { proposer, .. }, Command::AttachResponder(responder)) => {
                if let Some(proposer) = proposer {
                    self.phase = standby(
                        context,
                        Players {
                            proposer: *proposer,
                            responder,
                        },
                    )
                } else {
                    self.phase = Phase::WaitingForAttachment {
                        proposer: *proposer,
                        responder: Some(responder),
                    }
                }
            }
            (Phase::Standby { players }, Command::StartGame) => {
                // context.push_player_notify(players.proposer, UltimatumPlayerNotify::StartGame);
                // context.push_player_notify(players.responder, UltimatumPlayerNotify::StartGame);
                self.phase = Phase::Requesting {
                    players: *players,
                    proposal: None,
                };
            }
            (Phase::ProposalHidden { players, proposal }, Command::OpenProposal) => {
                self.phase = open_proposal(context, *players, *proposal)
            }
            (phase, command) => {
                // context.push_conductor_notify(UltimatumConductorNotify::Error(
                //     UltimatumError::UnexpectedConductorCommand {
                //         phase: phase.clone(),
                //         command,
                //     },
                // ))
            }
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
            (Phase::Requesting { proposal, players }, Command::UpdateProposal(new)) => {
                *proposal = Some(new);
                // if self.config.control_visibility == ControlVisibility::Realtime {
                //     context.push_player_notify(
                //         players.responder,
                //         UltimatumPlayerNotify::UpdateProposal(*proposal),
                //     );
                // }
            }
            (Phase::Requesting { players, .. }, Command::SubmitProposal(proposal)) => {
                match self.config.proposal_open_timing {
                    ProposalOpenTiming::Immediate => {
                        self.phase = open_proposal(context, *players, proposal)
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
                // context.push_player_notify(
                //     players.proposer,
                //     UltimatumPlayerNotify::Responded(response),
                // );
                self.phase = Phase::Result {
                    players: *players,
                    proposal: *proposal,
                    response,
                };
            }
            (phase, command) => {
                // context.push_conductor_notify(UltimatumConductorNotify::Error(
                //     UltimatumError::UnexpectedPlayerCommand {
                //         phase: phase.clone(),
                //         player_id,
                //         command,
                //     },
                // ))
            }
        }
    }

    fn handle_system_command(
        &mut self,
        _context: &mut impl GameContext<Self>,
        _command: SystemCommand,
    ) {
        todo!()
    }

    fn render_conductor(
        &self,
        context: &impl eagle_game::game::render_context::RenderContext,
    ) -> Self::ConductorView {
        todo!()
    }

    fn render_player(
        &self,
        context: &impl eagle_game::game::render_context::RenderContext,
        player_id: PlayerId,
    ) -> Self::PlayerView {
        UltimatumPlayer {}
    }
}
