pub mod bubble;
pub mod clients;
pub mod eff_handler;
pub mod effectful;
pub mod events;
pub mod game;
pub mod model;
pub mod room;

pub mod prelude {
    pub use crate::bubble::*;
    pub use crate::model::dispatcher::Dispatcher;
    pub use crate::events::GameCommand;
    pub use crate::game::Game;
    pub use crate::game::context::GameContext;
    pub use crate::game::handle::GameHandle;
    pub use crate::model::handle::ModelHandle;
    pub use crate::model::context::ModelContext;
    pub use crate::model::Model;
    pub use eagle_types::events::SystemCommand;
    pub type Map<K, V> = std::collections::BTreeMap<K, V>;
    pub type Set<T> = std::collections::BTreeSet<T>;
}
