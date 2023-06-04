use std::{collections::BTreeMap, any::Any, cell::RefCell, rc::Rc};

use eagle_types::ids::GameInstanceId;

use crate::{GameHandle, Game};

#[derive(Default)]
pub(crate) struct GameInstances {
    game_instances: BTreeMap<GameInstanceId, Rc<dyn Any>>,
}

impl GameInstances {
    pub fn new() -> Self {
        Self {
            game_instances: BTreeMap::new(),
        }
    }
    pub fn insert_game_instance<T: Game>(&mut self, handle: GameHandle<T>, game: T) {
        self.game_instances.insert(handle.game_instance_id, Rc::new(RefCell::new(game)));
    }
    pub fn get_game_instance<T: Game>(&self, handle: GameHandle<T>) -> impl std::ops::Deref<Target = T> + '_ {
        self.game_instances
            .get(&handle.game_instance_id)
            .unwrap()
            .downcast_ref::<RefCell<T>>()
            .unwrap()
            .borrow()
    }
    pub fn get_game_instance_mut<T: Game>(&mut self, handle: GameHandle<T>) -> Rc<RefCell<T>> {
        self.game_instances
            .get_mut(&handle.game_instance_id)
            .unwrap()
            .clone()
            .downcast::<RefCell<T>>()
            .unwrap()
    }
}
