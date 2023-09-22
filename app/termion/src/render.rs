use std::io::{stdout, Stdout, Write};

use snake::{
    game::Game,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldElement, FieldPoint},
};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen, ToAlternateScreen},
};

pub struct TermionRender {
    screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl TermionRender {
    pub fn new() -> TermionRender {
        let stdout = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();
        let mut screen = AlternateScreen::from(stdout);
        write!(screen, "{}{}", termion::cursor::Hide, ToAlternateScreen).unwrap();
        TermionRender { screen }
    }

    fn snake_tail(node: &Option<&SnakeNode>) -> char {
        if node.is_none() {
            return ' ';
        }
        return match node.unwrap().direction {
            Direction::Up | Direction::Down => 'Ꞌ',
            Direction::Right | Direction::Left => '-',
        };
    }

    fn snake_mounth_treat(node: &Option<&SnakeNode>) -> char {
        if node.is_none() {
            return ' ';
        }
        return match node.unwrap().direction {
            Direction::Up => 'v',
            Direction::Down => 'ʌ',
            Direction::Right => '<',
            Direction::Left => '>',
        };
    }

    fn snake_mounth(node: &Option<&SnakeNode>) -> char {
        if node.is_none() {
            return ' ';
        }
        return match node.unwrap().direction {
            Direction::Up => '⩀',
            Direction::Down => '⨃',
            Direction::Right => '⪾',
            Direction::Left => '⪽',
        };
    }
}

impl GameRender for TermionRender {
    fn snake_full(&mut self, snake: &Snake) {
        let nodes = &snake.nodes;
        let mut iter = nodes.iter();
        let tail = iter.next();
        write(TermionRender::snake_tail(&tail), &tail, &mut self.screen);

        for _i in 1..nodes.len() - 1 {
            write('*', &iter.next(), &mut self.screen);
        }
        let head = iter.next();
        write(TermionRender::snake_mounth(&head), &head, &mut self.screen);
        self.screen.flush().unwrap();
    }

    fn snake(&mut self, prev_tail: &Option<&SnakeNode>, game: &Game) {
        write(' ', prev_tail, &mut self.screen);

        let nodes = &game.snake.nodes;
        let tail = nodes.front();

        write(TermionRender::snake_tail(&tail), &tail, &mut self.screen);

        let mut iter = nodes.iter();
        let next_position = game.snake.next_head().position;
        let head = iter.next_back();
        if game.field[next_position.x][next_position.y] == FieldElement::Treat {
            write(
                TermionRender::snake_mounth_treat(&head),
                &head,
                &mut self.screen,
            );
        } else {
            write(TermionRender::snake_mounth(&head), &head, &mut self.screen);
        }
        write('*', &mut iter.next_back(), &mut self.screen);
        self.screen.flush().unwrap();
    }

    fn food(&mut self, p: &FieldPoint) {
        // write_point('🍎', p, &mut self.screen);
        write_point('@', p, &mut self.screen);
        self.screen.flush().unwrap();
    }
}

const X_OFFSET: u16 = 2;
const Y_OFFSET: u16 = 2;

pub fn write(
    c: char,
    node: &Option<&SnakeNode>,
    screen: &mut AlternateScreen<RawTerminal<Stdout>>,
) {
    match node {
        Some(n) => write_point(c, &n.position, screen),
        None => (),
    }
}

pub fn write_point(c: char, point: &FieldPoint, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    let x = point.x as u16;
    let y = point.y as u16;
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(x + X_OFFSET, y + Y_OFFSET),
        c
    )
    .unwrap();
}
