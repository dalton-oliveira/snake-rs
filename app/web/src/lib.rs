pub mod render;
mod sprites;
pub mod utils;

extern crate js_sys;
use snake::{
    game::Game,
    render::GameRender,
    types::{Direction, FieldPoint},
};
use sprites::SpritesBinary;
use wasm_bindgen::prelude::*;

use crate::utils::build_snake;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
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
    render: render::CanvasRender,
}
const WIDTH: u32 = 20;
const HEIGHT: u32 = 10;

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut render = render::CanvasRender::new(WIDTH, HEIGHT);
        let game = Game::new(&mut render, WIDTH as usize, HEIGHT as usize);
        Universe { game, render }
    }

    pub fn dot(&mut self, x: u32, y: u32) {
        self.render.dot(x, y);
    }

    pub fn draw(&mut self) {
        let snake = build_snake(
            vec![
                (7, 0, Direction::Left),  //
                (6, 0, Direction::Left),  //
                (5, 0, Direction::Left),  //
                (4, 1, Direction::Down),  //
                (3, 1, Direction::Right), //
            ],
            &self.game.snake.direction.max,
        );
        // log!("{:?}", snake);
        // self.render.snake_full(&snake);
    }

    pub fn horizontal_block(&mut self, x: u32, y: u32, block: u8) {
        self.render.horizontal_block(x, y, block);
    }

    pub fn vertical_block(&mut self, x: u32, y: u32, block: u8) {
        self.render.vertical_block(x, y, block);
    }

    pub fn key_down(&mut self, to: FrontKey) {
        match to {
            FrontKey::Up => self.game.snake.head_to(Direction::Up),
            FrontKey::Down => self.game.snake.head_to(Direction::Down),
            FrontKey::Left => self.game.snake.head_to(Direction::Left),
            FrontKey::Right => self.game.snake.head_to(Direction::Right),
        }
    }

    pub fn tick(&mut self) {
        self.game.tick(&mut self.render);
        // log!("{:?}", self.game.snake);
    }
}
