use super::{Model, handle::ModelHandle};

pub struct ModelContext<T: Model> {
    pub model_handle: ModelHandle<T>,
}

impl<T: Model> ModelContext<T> {
    pub fn propagate(&mut self, handle: ModelHandle<T>) {
        todo!()
    }
}
