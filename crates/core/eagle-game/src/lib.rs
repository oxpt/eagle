pub mod bubble;
pub mod clients;
mod command_history;
pub mod context;
pub mod eff_handler;
pub mod effectful;
pub mod events;
pub mod game;
pub mod game_handle;
mod game_instances;
mod notify_history;
pub mod room;

pub mod prelude {
    pub use crate::context::Context;
    pub use crate::events::GameCommand;
    pub use crate::game::{Game, Conductor, Frontend};
    pub use crate::game_handle::GameHandle;
    pub use crate::bubble::{CommandBubble, NotifyBubble};
    pub use eagle_types::events::SystemCommand;
    pub type Map<K, V> = std::collections::BTreeMap<K, V>;
    pub type Set<T> = std::collections::BTreeSet<T>;
}
