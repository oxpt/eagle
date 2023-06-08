use eagle_types::ids::GameInstanceId;

use crate::model::{context::ModelContextImpl, handle::ModelHandle, Model};

use self::model_instances::ModelInstances;

pub(crate) mod model_instances;

pub struct Screen<T: Model> {
    model_handle: ModelHandle<T>,
    model_instances: ModelInstances,
}

impl<T: Model> Screen<T> {
    pub fn new(game_instance_id: GameInstanceId) -> Self {
        let mut model_instances = ModelInstances::new();
        let model_handle = ModelHandle::new(game_instance_id);
        model_instances.insert_root(model_handle);
        Self {
            model_handle,
            model_instances,
        }
    }

    pub fn handle_notify(&mut self, notify: T::Notify) {
        let handle = self.model_handle;
        let game = self.model_instances.get_model_instance(handle);
        let mut ctx = ModelContextImpl {
            game_handle: handle,
            model_instances: &mut self.model_instances,
        };
        game.borrow_mut().handle_notify(&mut ctx, notify);
    }

    pub fn handle_input(&mut self, input: T::Input) -> T::Command {
        self.handle_input_of(self.model_handle, input)
    }

    pub(crate) fn handle_input_of<M: Model>(
        &mut self,
        handle: ModelHandle<M>,
        input: M::Input,
    ) -> T::Command
    {
        self.model_instances.handle_input::<T, M>(handle, input)
    }

    pub fn render(&mut self) -> T::View {
        self.render_of(self.model_handle)
    }

    pub(crate) fn render_of<M: Model>(&mut self, handle: ModelHandle<M>) -> M::View {
        self.model_instances.get_model_instance(handle).borrow().render()
    }
}
