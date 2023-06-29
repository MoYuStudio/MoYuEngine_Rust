
mod map;
mod player;

use map::Map;
use player::Player;

pub struct Game {
    map: Map,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new();
        let player = Player::new();

        Game { map, player }
    }

    pub fn run(&mut self) {
        // 游戏循环逻辑
    }
}
