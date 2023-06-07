use super::{handle::ModelHandle, Model};
use crate::{bubble::NotifyBubble, screen::model_instances::ModelInstances};

pub struct ModelContext<'a, T: Model> {
    pub(crate) game_handle: ModelHandle<T>,
    pub(crate) model_instances: &'a mut ModelInstances,
}

impl<'a, T: Model> ModelContext<'a, T> {
    pub fn propagate<M: Model>(&mut self, bubble: NotifyBubble<M>)
    where
        M::Command: Into<T::Command>,
    {
        let handle = ModelHandle::<M>::new(bubble.game_instance_id);
        let mut game = self
            .model_instances
            .get_model_instance_or_insert::<T, M>(self.game_handle, handle);
        let mut ctx = ModelContext {
            game_handle: handle,
            model_instances: self.model_instances,
        };
        game.borrow_mut().handle_notify(&mut ctx, bubble.notify);
    }
}
