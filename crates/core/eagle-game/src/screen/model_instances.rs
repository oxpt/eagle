use std::{any::Any, cell::RefCell, collections::BTreeMap, rc::Rc};

use eagle_types::ids::GameInstanceId;

use crate::model::{handle::ModelHandle, Model};

pub(crate) struct ModelInstances {
    instances: BTreeMap<GameInstanceId, Box<dyn Any>>,
}

struct ModelInstance<T: Model> {
    model: Rc<RefCell<T>>,
    command_intos: Vec<fn(Box<dyn Any>) -> Box<dyn Any>>,
}

impl ModelInstances {
    pub fn new() -> Self {
        Self {
            instances: BTreeMap::new(),
        }
    }

    pub fn insert_root<T: Model>(&mut self, handle: ModelHandle<T>) {
        self.instances.insert(
            handle.game_instance_id,
            Box::new(ModelInstance {
                model: Rc::new(RefCell::new(T::new())),
                command_intos: vec![],
            }),
        );
    }

    pub fn get_model_instance_or_insert<T: Model, Sub: Model>(
        &mut self,
        handle: ModelHandle<T>,
        sub: ModelHandle<Sub>,
    ) -> Rc<RefCell<Sub>>
    where
        Sub::Command: Into<T::Command>,
    {
        if self.instances.contains_key(&handle.game_instance_id) {
            self.instances
                .get(&handle.game_instance_id)
                .unwrap()
                .downcast_ref::<ModelInstance<Sub>>()
                .unwrap()
                .model
                .clone()
        } else {
            let mut intos = self
                .instances
                .get(&handle.game_instance_id)
                .unwrap()
                .downcast_ref::<ModelInstance<T>>()
                .unwrap()
                .command_intos
                .clone();
            intos.push(|any| {
                let sub_command = *any.downcast::<Sub::Command>().unwrap();
                let command: T::Command = sub_command.into();
                Box::new(command)
            });
            let model = Rc::new(RefCell::new(Sub::new()));
            self.instances.insert(
                sub.game_instance_id,
                Box::new(ModelInstance {
                    model: model.clone(),
                    command_intos: intos,
                }),
            );
            model
        }
    }

    pub(crate) fn get_model_instance<T: Model>(&self, handle: ModelHandle<T>) -> Rc<RefCell<T>> {
        self.instances
            .get(&handle.game_instance_id)
            .unwrap()
            .downcast_ref::<ModelInstance<T>>()
            .unwrap()
            .model
            .clone()
    }

    pub(crate) fn handle_input<T: Model, M: Model>(
        &self,
        handle: ModelHandle<M>,
        input: M::Input,
    ) -> T::Command {
        let instance = self
            .instances
            .get(&handle.game_instance_id)
            .unwrap()
            .downcast_ref::<ModelInstance<M>>()
            .unwrap();
        let model = instance.model.borrow_mut();
        let command = model.handle_input(input);
        let mut command: Box<dyn Any> = Box::new(command);
        for into in instance.command_intos.iter().rev() {
            command = into(command);
        }
        *command.downcast::<T::Command>().unwrap()
    }
}
