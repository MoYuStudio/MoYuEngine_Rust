
mod logic;
mod graphics;
mod utils;

use logic::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
