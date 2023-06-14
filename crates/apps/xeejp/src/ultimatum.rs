use eagle_types::ids::GameInstanceId;
use eagle_ultimatum::{config::UltimatumConfig, game::UltimatumGame};
use worker::*;

use crate::game::WorkerGame;

#[durable_object]
pub struct Ultimatum(WorkerGame<UltimatumGame>);

#[durable_object]
impl DurableObject for Ultimatum {
    fn new(state: State, env: Env) -> Self {
        Self(WorkerGame::new(state, env))
    }

    async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        self.0.fetch(req).await
    }
}
