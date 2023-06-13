use eagle_types::ids::PlayerId;

use crate::{prelude::GameHandle, room::game_instances::GameInstances};

use super::Game;

pub trait RenderContext {
    fn render_conductor<T: Game>(&self, handle: GameHandle<T>) -> T::ConductorView;
    fn render_player<T: Game>(&self, handle: GameHandle<T>, player_id: PlayerId) -> T::PlayerView;
}

pub struct RenderContextImpl<'a> {
    pub(crate) game_instances: &'a GameInstances,
}

impl RenderContext for RenderContextImpl<'_> {
    fn render_conductor<T: Game>(&self, handle: GameHandle<T>) -> T::ConductorView {
        let game = self.game_instances.get_game_instance(handle);
        let view = game.borrow().render_conductor(self);
        view
    }

    fn render_player<T: Game>(&self, handle: GameHandle<T>, player_id: PlayerId) -> T::PlayerView {
        let game = self.game_instances.get_game_instance(handle);
        let view = game.borrow().render_player(self, player_id);
        view
    }
}
