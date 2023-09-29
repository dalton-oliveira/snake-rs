use crate::{
    food::FoodField,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, Field, FoodType, GameState},
};

pub struct Game {
    pub width: u16,
    pub height: u16,
    pub score: u16,
    pub snake: Snake,
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
    pub fn new(game_render: &mut impl GameRender, config: GameConfig) -> Game {
        let (width, height) = config.dim;
        let mut field = Field::new(width, height);
        let snake = Snake::new(&mut field, config);
        let game = Game {
            score: 0,
            food: FoodField::new(),
            snake,
            field,
            state: GameState::None,
            width,
            height,
        };

        game_render.snake_full(&game);
        return game;
    }

    fn crawl(&mut self, game_render: &mut impl GameRender) {
        let next_head = self.snake.next_head();
        let SnakeNode { position: p, .. } = next_head;
        if self.field.filled(&p) {
            self.state = GameState::Over; //@todo comming soon...
            return;
        }
        self.snake.nodes.push_back(next_head);
        self.field.set(&p, true);
        match self.food.grab(&p) {
            None => {
                let tail = self.snake.nodes.pop_front().unwrap();
                self.field.set(&tail.position, false);
                game_render.snake(self);
            }
            Some(food) => {
                self.score += food.weight as u16;
                game_render.eat(self, &food);
                if food.shape == FoodType::Basic {
                    self.add_food(game_render);
                }
            }
        }
    }

    pub fn add_food(&mut self, game_render: &mut impl GameRender) {
        let max = (self.field.bit_set.len() - self.snake.nodes.len()) as u16;
        let food = self.food.add_food(max, &self.field);
        game_render.added_food(&food);

        if self.food.count % 6 == 0 {
            let food = self.food.add_special_food(max, &self.field);
            if let Some(food) = food {
                game_render.added_food(&food);
            }
        }
    }
    pub fn tick(&mut self, game_render: &mut impl GameRender) {
        if self.state == GameState::Quit {
            return;
        }
        self.food.tick(game_render);
        self.crawl(game_render);
    }
}
