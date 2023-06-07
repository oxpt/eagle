use eagle_game::prelude::Model;

pub struct NotifyLog<T: Model> {
    log: Vec<T::Notify>,
}
