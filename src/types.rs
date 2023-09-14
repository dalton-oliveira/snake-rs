use std::sync::{Arc, RwLock};

use crate::game::Game;

pub type GameArc = Arc<RwLock<Game>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldElement {
    Snake,
    Treat,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    None,
    Playing,
    Quit,
    Over,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FieldPoint {
    pub x: usize,
    pub y: usize,
}

impl FieldPoint {
    pub fn add(&self, direction: WrappableDirection) -> FieldPoint {
        let x = match direction.to {
            Direction::Right => self.x.wrapping_add(1).wrapping_rem(direction.max.x),
            Direction::Left => self
                .x
                .wrapping_add(direction.max.x)
                .wrapping_add_signed(-1)
                .wrapping_rem(direction.max.x),
            _ => self.x,
        };
        let y = match direction.to {
            Direction::Down => self.y.wrapping_add(1).wrapping_rem(direction.max.y),
            Direction::Up => self
                .y
                .wrapping_add(direction.max.y)
                .wrapping_add_signed(-1)
                .wrapping_rem(direction.max.y),
            _ => self.y,
        };
        FieldPoint { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
pub struct WrappableDirection {
    pub to: Direction,
    pub max: FieldPoint,
}
