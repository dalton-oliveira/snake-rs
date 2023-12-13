use std::io::{stdout, Stdout, Write};

use snake::{
    food::FoodField,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, Food, FoodType},
};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen, ToAlternateScreen},
};

pub struct TermionRender {
    screen: AlternateScreen<RawTerminal<Stdout>>,
    tail: Option<SnakeNode>,
}

impl GameRender for TermionRender {
    fn snake(&mut self, snake: &Snake, food_field: &FoodField) {
        let nodes = &snake.nodes;

        let mut iter = nodes.iter();
        let head = iter.next_back();
        if head.is_none() {
            return;
        }

        let mouth_open = food_field.has_at(&snake.next_head().position).is_some();
        let head = head.unwrap();
        self.replace_head(head, mouth_open);

        let mut node = head;
        for _i in 1..nodes.len() - 1 {
            node = iter.next_back().unwrap();
            write("*", node, &mut self.screen);
        }
        let tail = iter.next_back().unwrap();
        self.replace_tail(tail, node.direction);
    }

    fn food(&mut self, food: &Food) {
        let icon = match food.shape {
            FoodType::Basic => "@",
            _ => ":)",
        };
        write_point(icon, &food.location, &mut self.screen);
        self.screen.flush().unwrap();
    }

    fn score(&mut self, _score: u16) {
        // coming soon...
    }
}
impl Default for TermionRender {
    fn default() -> Self {
        let stdout = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();
        let mut screen = stdout;
        write!(screen, "{}{}", termion::cursor::Hide, ToAlternateScreen).unwrap();
        TermionRender { screen, tail: None }
    }
}
impl TermionRender {
    pub fn clear(&mut self) {
        write!(self.screen, "{}", termion::clear::All).unwrap();
    }
    pub fn show_cursor(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
        self.screen.flush().unwrap();
    }
    fn save_tail(&mut self, tail: SnakeNode) {
        self.tail = Some(tail);
    }
    fn clear_tail(&mut self) {
        if let Some(tail) = self.tail {
            write(" ", &tail, &mut self.screen);
        }
    }
    fn snake_tail(node: &SnakeNode) -> &str {
        match node.direction {
            Direction::Up | Direction::Down => "Ꞌ",
            Direction::Right | Direction::Left => "-",
        }
    }

    fn snake_mounth_treat(node: &SnakeNode) -> &str {
        match node.direction {
            Direction::Up => "v",
            Direction::Down => "ʌ",
            Direction::Right => "<",
            Direction::Left => ">",
        }
    }

    fn snake_mounth(node: &SnakeNode) -> &str {
        match node.direction {
            Direction::Up => "⩀",
            Direction::Down => "⨃",
            Direction::Right => "⪾",
            Direction::Left => "⪽",
        }
    }
    fn replace_tail(&mut self, tail: &SnakeNode, direction: Direction) {
        self.clear_tail();

        let tail = SnakeNode {
            position: tail.position,
            direction,
            stuffed: false,
        };

        self.save_tail(tail);

        write(TermionRender::snake_tail(&tail), &tail, &mut self.screen);
    }

    fn replace_head(&mut self, head: &SnakeNode, mouth_open: bool) {
        let sprite = match mouth_open {
            true => TermionRender::snake_mounth_treat(head),
            false => TermionRender::snake_mounth(head),
        };
        write(sprite, head, &mut self.screen);
    }
}

const X_OFFSET: u16 = 2;
const Y_OFFSET: u16 = 2;

pub fn write(c: &str, node: &SnakeNode, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write_point(c, &node.position, screen);
}

pub fn write_point(c: &str, point: &FieldPoint, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    let x = point.x;
    let y = point.y;
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(x + X_OFFSET, y + Y_OFFSET),
        c
    )
    .unwrap();
}
