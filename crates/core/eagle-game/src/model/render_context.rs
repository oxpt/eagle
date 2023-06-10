use super::{handle::ModelHandle, Model};
use crate::screen::model_instances::ModelInstances;

pub(crate) struct RenderContextImpl<'a> {
    pub(crate) model_instances: &'a ModelInstances,
}

pub trait RenderContext {
    fn render<T: Model>(&self, handle: ModelHandle<T>) -> T::View;
}

impl<'a> RenderContext for RenderContextImpl<'a> {
    fn render<T: Model>(&self, handle: ModelHandle<T>) -> T::View {
        let game = self.model_instances.get_model_instance::<T>(handle);
        let ctx = RenderContextImpl {
            model_instances: self.model_instances,
        };
        let view = game.borrow().render(&ctx);
        view
    }
}
