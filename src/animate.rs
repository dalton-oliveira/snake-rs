use std::io::Stdout;

use termion::{raw::RawTerminal, screen::AlternateScreen};

use crate::{
    game::Game,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldElement},
    utils,
};

pub fn snake_full(snake: &Snake, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    let nodes = &snake.nodes;
    let mut iter = nodes.iter();
    utils::write('-', &iter.next(), screen);
    for _i in 1..nodes.len() - 1 {
        utils::write('*', &iter.next(), screen);
    }
    utils::write('=', &iter.next(), screen);
}

pub fn snake(
    prev_tail: &Option<&SnakeNode>,
    game: &Game,
    screen: &mut AlternateScreen<RawTerminal<Stdout>>,
) {
    utils::write(' ', prev_tail, screen);

    let nodes = &game.snake.nodes;
    let tail = nodes.back();

    utils::write(snake_tail(&tail), &tail, screen);

    let mut iter = nodes.iter();
    let next_position = game.snake.next_head().position;
    let head = iter.next();
    if game.field[next_position.x][next_position.y] == FieldElement::Treat {
        utils::write(snake_mounth_treat(&head), &head, screen);
    } else {
        utils::write(snake_mounth(&head), &head, screen);
    }

    utils::write('*', &mut iter.next(), screen);
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
