use eagle_types::ids::GameInstanceId;
use serde::{Serialize, Deserialize};

use super::Model;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// This is used to a view model owns sub games' view models.
pub struct ModelHandle<T: Model> {
    pub game_instance_id: GameInstanceId,
    _phantom: std::marker::PhantomData<T>,
}

impl <T: Model> ModelHandle<T> {
    pub(crate) fn new(game_instance_id: GameInstanceId) -> Self {
        Self {
            game_instance_id,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl <T: Model> Clone for ModelHandle<T> {
    fn clone(&self) -> Self {
        Self {
            game_instance_id: self.game_instance_id,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl <T: Model> Copy for ModelHandle<T> {}
