use snake::{
    game::GameData,
    render::GameRender,
    types::{FoodType, GameConfig},
    utils::decode,
};
use wasm_bindgen::prelude::*;

use crate::{render::BinaryRender, screen::CanvasScreen};

#[wasm_bindgen]
pub struct GameScene {
    snake_id: Option<u16>,
    data: Option<GameData>,
    render: BinaryRender,
}

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl Default for GameScene {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl GameScene {
    pub fn new() -> GameScene {
        let config = GameConfig::default();
        let (width, height) = config.dim;
        let screen = CanvasScreen {};
        let render = BinaryRender::new(width, height, Box::new(screen));
        GameScene {
            data: None,
            render,
            snake_id: None,
        }
    }
    pub fn snake_id(&mut self, data: Vec<u8>) {
        let (snake_id, _size): (u16, usize) = decode(&data).unwrap();
        self.snake_id = Some(snake_id);
    }

    pub fn draw(&mut self) {
        unsafe { clearField() };
        if let Some(data) = &self.data {
            for (_id, snake) in data.snakes.iter() {
                self.render.snake(snake, &data.food);
                if self.snake_id.eq(&Some(snake.id)) {
                    self.render.score(snake.score);
                }
            }
            let mut special_idx = 0;
            for food in data.food.foods.iter() {
                self.render.food(food);
                if food.shape != FoodType::Basic {
                    self.render.draw_food_ticker(food, special_idx);
                    special_idx += 1;
                }
            }
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        let (data, _size): (GameData, usize) = decode(&data).unwrap();
        let (width, height) = data.config.dim;
        let screen = CanvasScreen {};
        self.render = BinaryRender::new(width, height, Box::new(screen));
        self.data = Some(data);
    }
}

#[link(wasm_import_module = "/canvas/field.js")]
extern "C" {
    fn clearField();
}
