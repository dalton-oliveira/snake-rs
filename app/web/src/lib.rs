pub mod render;
mod sprites;
pub mod utils;

extern crate js_sys;
use snake::{
    game::{Game, GameConfig},
    types::Direction,
};
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub enum FrontKey {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
pub struct Universe {
    game: Game,
    render: render::BinaryRender,
    snake_id: usize,
}
const CONFIG: GameConfig = GameConfig {
    size: 5,
    start: (1, 0),
    dim: (10, 10),
    direction: Direction::Right,
};

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let (width, height) = CONFIG.dim;
        let mut render = render::BinaryRender::new(width, height);

        let mut game = Game::new(CONFIG);
        let snake_id = game.add_snake(&mut render);
        game.add_food(&mut render);
        Universe {
            game,
            render,
            snake_id,
        }
    }

    pub fn key_down(&mut self, to: FrontKey) {
        let game = &mut self.game;
        match to {
            FrontKey::Up => game.head_to(self.snake_id, Direction::Up),
            FrontKey::Down => game.head_to(self.snake_id, Direction::Down),
            FrontKey::Left => game.head_to(self.snake_id, Direction::Left),
            FrontKey::Right => game.head_to(self.snake_id, Direction::Right),
        };
    }

    pub fn tick(&mut self) {
        self.game.tick(&mut self.render);
    }
}
