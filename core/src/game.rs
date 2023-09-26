use crate::{
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldElement, FieldPoint, GameState, SUPER_FOOD},
};
use rand::{seq::SliceRandom, Rng};

#[derive(Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub score: usize,
    pub snake: Snake,
    pub food_count: usize,
    // @todo use FixedBitSet
    pub field: Vec<Vec<FieldElement>>,
    pub state: GameState,
}

pub struct GameConfig {
    pub size: usize,
    pub start: (usize, usize),
    pub dim: (usize, usize),
    pub direction: Direction,
}

impl Game {
    pub fn new(game_render: &mut impl GameRender, config: GameConfig) -> Game {
        let (width, height) = config.dim;
        let mut field: Vec<Vec<FieldElement>> = vec![vec![FieldElement::Empty; width]; height];
        let snake = Snake::new(&mut field, config);
        let game = Game {
            score: 0,
            food_count: 0,
            snake: snake,
            field: field,
            state: GameState::None,
            width,
            height,
        };

        game_render.snake_full(&game);
        return game;
    }

    fn crawl(&mut self, game_render: &mut impl GameRender) {
        // @todo maybe move this to the snake
        let next_head = self.snake.next_head();
        let SnakeNode { position: p, .. } = next_head;
        match self.field[p.x][p.y] {
            FieldElement::Empty => {
                self.snake.nodes.push_back(next_head);
                self.field[p.x][p.y] = FieldElement::Snake;
                let tail = self.snake.nodes.pop_front().unwrap();
                self.field[tail.position.x][tail.position.y] = FieldElement::Empty;
                game_render.snake(self);
            }
            FieldElement::Snake => self.state = GameState::Over, // @todo coming soon..
            x => {
                //@todo sum points, check for game over
                self.score += Game::element_score(x);
                self.food_count += 1;
                self.snake.nodes.push_back(next_head);
                game_render.eat(self, p);
                self.field[p.x][p.y] = FieldElement::Snake;

                if self.food_count % 6 == 0 {
                    let mut rng_foods = SUPER_FOOD.clone();
                    rng_foods.shuffle(&mut rand::thread_rng());
                    self.add_food(rng_foods[0], game_render);
                } else {
                    self.add_food(FieldElement::Treat, game_render);
                }
            }
        }
    }

    fn element_score(element: FieldElement) -> usize {
        return match element {
            FieldElement::Treat => 8,
            FieldElement::Empty => 0,
            FieldElement::Snake => 0,
            _ => 45,
        };
    }
    pub fn add_food(&mut self, food: FieldElement, game_render: &mut impl GameRender) {
        // let p = FieldPoint { x: 8, y: 0 };
        // self.field[p.x][p.y] = food;
        // game_render.food(self, p);

        let available = (self.width * self.height) - self.snake.nodes.len();
        let rand_pos = rand::thread_rng().gen_range(0..available - 1);
        let mut pos = 0;
        'outer_loop: for x in 0..self.field.len() {
            for y in 0..self.field[x].len() {
                if self.field[x][y] == FieldElement::Empty {
                    pos += 1;
                }
                if pos > rand_pos {
                    self.field[x][y] = food;
                    game_render.food(self, FieldPoint { x, y });
                    break 'outer_loop;
                }
            }
        }
    }

    pub fn tick(&mut self, game_render: &mut impl GameRender) {
        match self.state {
            GameState::Quit => return,
            _ => self.crawl(game_render),
        }
    }
}
