pub mod render;
mod sprites;
pub mod utils;

extern crate js_sys;
use snake::{
    game::{Game, GameConfig},
    types::{Direction, GameState},
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

        let mut game = Game::new(&mut render, CONFIG);
        game.add_food(&mut render);
        Universe { game, render }
    }

    pub fn key_down(&mut self, to: FrontKey) -> bool {
        let snake = &mut self.game.snake;
        return match to {
            FrontKey::Up => snake.head_to(Direction::Up),
            FrontKey::Down => snake.head_to(Direction::Down),
            FrontKey::Left => snake.head_to(Direction::Left),
            FrontKey::Right => snake.head_to(Direction::Right),
        };
    }

    pub fn tick(&mut self) {
        self.game.tick(&mut self.render);
    }
}
