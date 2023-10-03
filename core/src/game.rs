use std::collections::HashMap;

use bincode::{config, decode_from_slice, encode_to_vec};

use crate::{
    food::FoodField,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, Field, FoodType, GameConfig, GameState},
};

#[derive(bincode::Encode, bincode::Decode, Debug)]
pub struct GameData {
    pub config: GameConfig,
    pub snakes: HashMap<u16, Snake>,
    pub food: FoodField,
    pub state: GameState,
}

impl GameData {
    pub fn from_game(game: &Game) -> GameData {
        GameData {
            config: game.config.clone(),
            snakes: game.snakes.clone(),
            food: game.food.clone(),
            state: game.state.clone(),
        }
    }
}

pub struct Game {
    pub config: GameConfig,
    pub snakes: HashMap<u16, Snake>,
    pub field: Field,
    pub food: FoodField,
    pub state: GameState,
}

impl Game {
    pub fn new(config: GameConfig) -> Game {
        let (width, height) = config.dim;
        let field = Field::new(width, height);
        let game = Game {
            config,
            food: FoodField::new(),
            snakes: HashMap::new(),
            field,
            state: GameState::None,
        };
        return game;
    }

    pub fn encode_game_data(&self) -> Vec<u8> {
        encode_to_vec(GameData::from_game(self), config::standard()).unwrap()
    }

    pub fn set_game_data(&mut self, data: Vec<u8>) {
        let (data, _size): (GameData, usize) =
            decode_from_slice(&data[..], config::standard()).unwrap();

        let (width, height) = data.config.dim;
        let mut field = Field::new(width, height);
        for (_id, snake) in data.snakes.iter() {
            for node in snake.nodes.iter() {
                field.set(&node.position, true);
            }
        }
        self.config = data.config;
        self.food = data.food;
        self.snakes = data.snakes;
        self.field = field;
        self.state = data.state;
    }

    // @todo spot an empty continuous space to fit the snake
    pub fn add_snake(&mut self) -> u16 {
        let mut config = self.config.clone();
        config.start = (0, self.snakes.len() as u16);

        let id = (self.snakes.len() + 1) as u16;
        let snake = Snake::new(&mut self.field, &config, id);
        self.snakes.insert(id, snake);
        return id;
    }

    pub fn remove_snake(&mut self, snake_id: u16) {
        //@todo clear rendering coming soon..
        if let Some(snake) = self.snakes.remove(&snake_id) {
            for node in snake.nodes.iter() {
                self.field.set(&node.position, false);
            }
        }
    }

    pub fn add_food(&mut self) {
        let max = self.max();
        self.food.add_food(max, &self.field);
    }

    fn crawl(&mut self) {
        for snake in self.snakes.values_mut() {
            let mut next_head = snake.next_head();
            let SnakeNode { position: p, .. } = next_head;
            if self.field.filled(&p) {
                // self.state = GameState::Over; //@todo comming soon...
                continue;
            }
            let nodes = &mut snake.nodes;
            self.field.set(&p, true);
            match self.food.grab(&p) {
                Some(food) => {
                    next_head.stuffed = true;
                    snake.score += food.weight as u16;
                }
                None => {
                    let tail = nodes.pop_front().unwrap();
                    self.field.set(&tail.position, false);
                }
            }
            nodes.push_back(next_head);
        }
    }

    pub fn add_missing_food(&mut self) {
        let mut required_foods = self.snakes.len();
        for food in self.food.foods.iter() {
            if food.shape == FoodType::Basic && required_foods > 0 {
                required_foods -= 1;
            }
        }
        for _ in 0..required_foods {
            self.food.add_food(self.max(), &self.field);
        }
    }

    pub fn head_to(&mut self, snake_id: u16, to: Direction) {
        if let Some(snake) = self.snakes.get_mut(&snake_id) {
            snake.head_to(to);
        }
    }

    fn max(&self) -> u16 {
        let mut max: usize = 0;
        for snake in self.snakes.values() {
            max += snake.nodes.len();
        }
        return (self.field.bit_set.len() - max) as u16;
    }

    pub fn draw(&mut self, render: &mut impl GameRender) {
        // @todo use snake_id as render param
        for (_id, snake) in self.snakes.iter() {
            render.snake(&snake, &self.food);
        }
        for food in self.food.foods.iter() {
            render.food(&food);
        }
    }

    pub fn tick(&mut self) {
        if self.state == GameState::Quit {
            return;
        }
        self.food.tick();
        self.crawl();
    }
}
