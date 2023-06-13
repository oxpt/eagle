pub mod clients;
pub mod eff_handler;
pub mod effectful;
pub mod events;
pub mod game;
pub mod room;

pub mod prelude {
    pub use crate::events::GameCommand;
    pub use crate::game::context::GameContext;
    pub use crate::game::handle::GameHandle;
    pub use crate::game::Game;
    pub use eagle_types::events::SystemCommand;
    pub use eagle_types::ids::PlayerId;
    pub type Map<K, V> = std::collections::BTreeMap<K, V>;
    pub type Set<T> = std::collections::BTreeSet<T>;
}
