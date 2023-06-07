use std::marker::PhantomData;

use eagle_types::ids::GameInstanceId;

use crate::{game::Game, model::handle::ModelHandle};

#[derive(Debug)]
pub struct GameHandle<T: Game> {
    pub game_instance_id: GameInstanceId,
    phantom: PhantomData<T>,
}

impl <T: Game> GameHandle<T> {
    pub(crate) fn new(game_instance_id: GameInstanceId) -> Self {
        Self {
            game_instance_id,
            phantom: PhantomData,
        }
    }

    pub fn to_conductor_model_handle(&self) -> ModelHandle<T::Conductor> {
        ModelHandle::new(self.game_instance_id)
    }

    pub fn to_player_model_handle(&self) -> ModelHandle<T::Player> {
        ModelHandle::new(self.game_instance_id)
    }
}

impl <T: Game> Clone for GameHandle<T> {
    fn clone(&self) -> Self {
        Self {
            game_instance_id: self.game_instance_id,
            phantom: PhantomData,
        }
    }
}

impl <T: Game> Copy for GameHandle<T> {}
