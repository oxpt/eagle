mod game;
mod context;
mod game_handle;
mod room;
mod clients;
mod command_history;
mod effectful;
mod game_instances;
mod eff_handler;
mod events;
mod bubbled;
mod notify_history;

pub use game::Game;
pub use context::Context;
pub use game_handle::GameHandle;
pub use eagle_types::events::SystemCommand;
pub use events::GameCommand;

#[cfg(feature = "server")]
mod server {
    pub use crate::room::Room;
    pub use crate::eff_handler::EffHandler;
    pub use crate::effectful::Effectful;
    pub use crate::clients::Clients as ClientsRef;
}
#[cfg(feature = "server")]
pub use server::*;
