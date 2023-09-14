use std::io::Stdout;

use crate::{snake::SnakeNode, types::FieldPoint};
use std::io::Write;
use termion::{raw::RawTerminal, screen::AlternateScreen};
const X_OFFSET: u16 = 2;
const Y_OFFSET: u16 = 2;

pub fn write(
    c: char,
    node: &Option<&SnakeNode>,
    screen: &mut AlternateScreen<RawTerminal<Stdout>>,
) {
    match node {
        Some(n) => write_point(c, n.position, screen),
        None => (),
    }
}

pub fn write_point(c: char, point: FieldPoint, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
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
