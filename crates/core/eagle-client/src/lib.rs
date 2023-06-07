use eagle_game::prelude::Game;

pub struct Client<T: Game> {
    clients: Clients<C>,
    room: Room<T>,
}
