pub mod context;
pub mod handle;
pub mod render_context;

use erased_serde::Serialize;
use serde::de::DeserializeOwned;

use self::{context::ModelContext, render_context::RenderContext};

pub trait Model: Sized + 'static {
    type View: 'static;
    type Input: 'static;
    type Notify: Clone + Serialize + DeserializeOwned + 'static;
    type Command: Clone + Serialize + DeserializeOwned + 'static;

    fn new() -> Self;

    // The context is used for propagate notify to sub games;
    // Other than this must not use mut reference to enable replayability of the view model.
    fn handle_notify(&mut self, context: &mut impl ModelContext<Self>, notify: Self::Notify);

    // Rendering must not know about sub games because the knowing is done by the game.
    fn render(&self, context: &impl RenderContext) -> Self::View;

    fn handle_input(&self, input: Self::Input) -> Self::Command;
}
