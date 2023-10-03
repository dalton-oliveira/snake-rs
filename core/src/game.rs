use std::collections::HashMap;

use crate::{
    food::FoodField,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, Field, FoodType, GameState},
};

pub struct Game {
    pub config: GameConfig,
    pub score: u16,
    pub snakes: HashMap<usize, Snake>,
    pub field: Field,
    pub food: FoodField,
    pub state: GameState,
}

pub struct GameConfig {
    pub size: u16,
    pub start: (u16, u16),
    pub dim: (u16, u16),
    pub direction: Direction,
}

impl Game {
    pub fn new(config: GameConfig) -> Game {
        let (width, height) = config.dim;
        let field = Field::new(width, height);
        let game = Game {
            config,
            score: 0,
            food: FoodField::new(),
            snakes: HashMap::new(),
            field,
            state: GameState::None,
        };
        return game;
    }

    pub fn add_snake(&mut self, game_render: &mut impl GameRender) -> usize {
        let snake = Snake::new(&mut self.field, &self.config);
        let id = self.snakes.len();
        self.snakes.insert(id, snake);

        let snake = &self.snakes.get(&id).unwrap();
        game_render.snake_full(&snake, &self.food);
        return id;
    }

    pub fn add_food(&mut self, game_render: &mut impl GameRender) {
        let max = self.max();
        self.food.add_food(max, &self.field, game_render);
    }

    fn crawl(&mut self, game_render: &mut impl GameRender) {
        let max = self.max();
        for snake in self.snakes.values_mut() {
            let next_head = snake.next_head();
            let SnakeNode { position: p, .. } = next_head;
            if self.field.filled(&p) {
                // self.state = GameState::Over; //@todo comming soon...
                return;
            }
            let nodes = &mut snake.nodes;
            nodes.push_back(next_head);
            self.field.set(&p, true);
            match self.food.grab(&p, game_render) {
                None => {
                    let tail = nodes.pop_front().unwrap();
                    self.field.set(&tail.position, false);
                    game_render.crawl(&snake, &self.food);
                }
                Some(food) => {
                    self.score += food.weight as u16;
                    game_render.update_score(self.score);
                    game_render.grow(&snake, &self.food);
                    if food.shape == FoodType::Basic {
                        self.food.add_food(max, &self.field, game_render);
                    }
                }
            }
        }
    }

    pub fn head_to(&mut self, snake_id: usize, to: Direction) {
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

    pub fn tick(&mut self, game_render: &mut impl GameRender) {
        if self.state == GameState::Quit {
            return;
        }
        self.food.tick(game_render);
        self.crawl(game_render);
    }
}
