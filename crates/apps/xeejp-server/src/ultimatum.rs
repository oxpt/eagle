use eagle_ultimatum::game::UltimatumGame;
use worker::*;

use crate::game::WorkerGame;

#[durable_object]
pub struct Ultimatum2023(WorkerGame<UltimatumGame>);

#[durable_object]
impl DurableObject for Ultimatum2023 {
    fn new(state: State, env: Env) -> Self {
        Self(WorkerGame::new(state, env))
    }

    async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        self.0.fetch(req).await
    }
}
