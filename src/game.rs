use crate::{
    animate,
    snake::{Snake, SnakeNode},
    types::{FieldElement, FieldPoint, GameState},
    utils,
};
use rand::Rng;
use std::io::Stdout;
use std::io::Write;
use termion::{raw::RawTerminal, screen::AlternateScreen};

#[derive(Debug)]
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub snake: Snake,
    pub field: Vec<Vec<FieldElement>>,
    pub state: GameState,
}

impl Game {
    pub fn new(screen: &mut AlternateScreen<RawTerminal<Stdout>>) -> Game {
        let width = 20;
        let height = 20;
        // @todo move to a game_scene concept
        let mut field: Vec<Vec<FieldElement>> = vec![vec![FieldElement::Empty; height]; width];

        let size: usize = 4;
        let snake = Snake::new(&mut field, size);
        animate::snake_full(&snake, screen);

        return Game {
            snake: snake,
            field: field,
            state: GameState::None,
            width,
            height,
        };
    }

    fn crawl(&mut self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        let next_head = self.snake.next_head();
        let SnakeNode { position, .. } = next_head;
        match self.field[position.x][position.y] {
            FieldElement::Empty => {
                self.snake.nodes.push_front(next_head);
                self.field[position.x][position.y] = FieldElement::Snake;
                let tail = self.snake.nodes.pop_back().unwrap();
                self.field[tail.position.x][tail.position.y] = FieldElement::Empty;
                animate::snake(&Some(&tail), self, screen);
            }
            FieldElement::Treat => {
                //@todo sum points, check for game over
                self.snake.nodes.push_front(next_head);
                self.field[position.x][position.y] = FieldElement::Snake;

                self.add_food(screen);
                animate::snake(&None, self, screen);
            }
            FieldElement::Snake => self.state = GameState::Over,
        }
    }

    pub fn add_food(&mut self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        let available = (self.width * self.height) - self.snake.nodes.len();
        let mut rng = rand::thread_rng();
        let rand_pos = rng.gen_range(0..available - 1);
        let mut pos = 0;
        for x in 0..self.field.len() {
            for y in 0..self.field[x].len() {
                if self.field[x][y] == FieldElement::Empty {
                    pos += 1;
                }
                if pos > rand_pos {
                    self.field[x][y] = FieldElement::Treat;
                    utils::write_point('🍎', FieldPoint { x, y }, screen);
                    write!(
                        screen,
                        "{}{}treat: {},{}",
                        termion::cursor::Goto(1, 22),
                        termion::clear::CurrentLine,
                        x,
                        y,
                    )
                    .unwrap();

                    break;
                }
            }
            if pos > rand_pos {
                break;
            }
        }
    }

    pub fn tick(&mut self, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
        if self.state == GameState::Quit {
            return;
        }
        self.crawl(screen);
    }
}
