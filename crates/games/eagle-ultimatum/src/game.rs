#![allow(dead_code)]

use eagle_game::{game::render_context::RenderContext, prelude::*};

use crate::{
    conductor_model::UltimatumConductor,
    config::UltimatumConfig,
    events::{UltimatumConductorCommand, UltimatumPlayerCommand},
    phase::Phase,
    player_model::UltimatumPlayer,
    types::{ControlVisibility, Proposal, ProposalOpenTiming, Response},
};

#[derive(Debug, Default)]
pub struct UltimatumGame {
    config: UltimatumConfig,
    phase: Phase,
    conductor: UltimatumConductor,
    proposer: Option<PlayerId>,
    responder: Option<PlayerId>,
    proposal: Option<Proposal>,
    response: Option<Response>,
    errors: Vec<String>,
}

impl Game for UltimatumGame {
    type Config = UltimatumConfig;
    type ConductorCommand = UltimatumConductorCommand;
    type PlayerCommand = UltimatumPlayerCommand;
    type ConductorView = UltimatumConductor;
    type PlayerView = UltimatumPlayer;

    fn new(config: Self::Config) -> Self {
        Self::default()
    }

    fn name() -> &'static str {
        "Ultimatum"
    }

    fn handle_conductor_command(
        &mut self,
        _context: &mut impl GameContext<Self>,
        command: Self::ConductorCommand,
    ) {
        use UltimatumConductorCommand as Command;
        match (&mut self.phase, command) {
            (Phase::WaitingForAttachment, Command::AttachProposer(proposer)) => {
                self.proposer = Some(proposer);
                if self.responder.is_some() {
                    self.phase = Phase::Standby;
                }
            }
            (Phase::WaitingForAttachment, Command::AttachResponder(responder)) => {
                self.responder = Some(responder);
                if self.proposer.is_some() {
                    self.phase = Phase::Standby;
                }
            }
            (Phase::Standby, Command::StartGame) => {
                self.phase = Phase::Proposing;
            }
            (Phase::ProposalHidden, Command::OpenProposal) => {
                self.phase = Phase::Responding;
            }
            (phase, command) => {
                self.errors.push(format!(
                    "Unexpected conductor command: {:?} in phase {:?}",
                    command, phase
                ));
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
            (Phase::Proposing, Command::UpdateProposal(new)) => {
                self.proposal = Some(new);
            }
            (Phase::Proposing, Command::SubmitProposal(proposal)) => {
                self.proposal = Some(proposal);
                match self.config.proposal_open_timing {
                    ProposalOpenTiming::Immediate => {
                        self.phase = Phase::Responding;
                    }
                    ProposalOpenTiming::ByConductor => {
                        self.phase = Phase::ProposalHidden;
                    }
                }
            }
            (Phase::Responding, Command::Respond(response)) => {
                self.response = Some(response);
                self.phase = Phase::Result;
            }
            (phase, command) => {
                self.errors.push(format!(
                    "Unexpected player command: {:?} in phase {:?}",
                    command, phase
                ));
            }
        }
    }

    fn handle_system_command(
        &mut self,
        _context: &mut impl GameContext<Self>,
        _command: SystemCommand,
    ) {
        // Nothing for now
    }

    fn render_conductor(&self, _context: &impl RenderContext) -> Self::ConductorView {
        UltimatumConductor {
            phase: self.phase.clone(),
            proposal: self.proposal,
            response: self.response,
            errors: self.errors.clone(),
        }
    }

    fn render_player(
        &self,
        _context: &impl RenderContext,
        player_id: PlayerId,
    ) -> Self::PlayerView {
        if self.proposer == Some(player_id) {
            match self.phase {
                Phase::WaitingForAttachment => UltimatumPlayer::Standby,
                Phase::Standby => UltimatumPlayer::Standby,
                Phase::Proposing => UltimatumPlayer::Proposing,
                Phase::ProposalHidden => UltimatumPlayer::WaitingForResponse,
                Phase::Responding => UltimatumPlayer::Responding {
                    proposal: self.proposal.unwrap(),
                },
                Phase::Result => UltimatumPlayer::Result {
                    proposal: self.proposal.unwrap(),
                    response: self.response.unwrap(),
                },
            }
        } else if self.responder == Some(player_id) {
            match self.phase {
                Phase::WaitingForAttachment => UltimatumPlayer::Standby,
                Phase::Standby => UltimatumPlayer::Standby,
                Phase::Proposing => match self.config.control_visibility {
                    ControlVisibility::Realtime => UltimatumPlayer::WaitingForProposal {
                        realtime_proposal: self.proposal,
                    },
                    ControlVisibility::Hidden => UltimatumPlayer::WaitingForProposal {
                        realtime_proposal: None,
                    },
                },
                Phase::ProposalHidden => UltimatumPlayer::WaitingForProposal {
                    realtime_proposal: None,
                },
                Phase::Responding => UltimatumPlayer::Responding {
                    proposal: self.proposal.unwrap(),
                },
                Phase::Result => UltimatumPlayer::Result {
                    proposal: self.proposal.unwrap(),
                    response: self.response.unwrap(),
                },
            }
        } else {
            UltimatumPlayer::Standby
        }
    }
}
