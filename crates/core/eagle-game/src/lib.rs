mod game;
mod context;
mod game_handle;
mod room;
pub mod clients;
mod event_history;
mod effectful;
mod game_instances;
mod eff_handler;

pub use game::Game;
pub use context::Context;
pub use game_handle::GameHandle;

#[cfg(feature = "server")]
mod server {
    pub use crate::room::Room;
    pub use crate::eff_handler::EffHandler;
}
#[cfg(feature = "server")]
pub use server::*;
