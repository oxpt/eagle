use eagle_game::{prelude::Model, screen::Screen};
use eagle_types::events::NotifyIndex;

/// A struct to:
/// - [ ] resend command
// This implementation should be small and simple because server-side do lots of work.
// Also most works are done by specific implementation of client, so this struct does just a
// little.
pub struct GameClient<T: Model> {
    screen: Screen<T>,
}

impl<T: Model> GameClient<T> {
    pub fn new(screen: Screen<T>) -> Self {
        Self { screen }
    }

    pub fn handle_notify(&mut self, index: NotifyIndex, notify: T::Notify) {
        // TODO: skip handled notify
        self.screen.handle_notify(notify);
    }
}
